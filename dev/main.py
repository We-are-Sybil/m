from typing import List, Optional
import geopandas as gpd
from shapely.geometry import Point

import httpx
from pydantic import BaseModel, Field, field_validator, ConfigDict


class Coordinate(BaseModel):
    coordinates: List[float] = Field(..., description="Coordinates in [longitude, latitude] format")

    @property
    def lat(self) -> float:
        return self.coordinates[1]

    @property
    def lon(self) -> float:
        return self.coordinates[0]

    def to_3857(self) -> "Coordinate":
        """
        Reproject this point from EPSG:4326 to EPSG:3857 (Web Mercator)
        and return a new Coordinate holding the [x, y] in meters.
        """
        # Create a GeoSeries with the single point in lon/lat
        pt = Point(self.lon, self.lat)
        geo = gpd.GeoSeries([pt], crs="EPSG:4326")
        # Reproject
        webmerc = geo.to_crs("EPSG:3857")
        # Extract x, y
        x, y = webmerc.geometry.iloc[0].x, webmerc.geometry.iloc[0].y
        # Return a new Coordinate—in this context, coordinates=[x, y]
        return Coordinate(coordinates=[x, y])


class Lane(BaseModel):
    valid: bool
    indications: List[str]


class Intersection(BaseModel):
    out: Optional[int] = None
    in_: Optional[int] = Field(None, alias="in")
    entry: List[bool]
    bearings: List[int]
    location: Coordinate
    lanes: Optional[List[Lane]] = None

    model_config = ConfigDict(populate_by_name=True)

    @field_validator("location", mode="before")
    def _parse_location(cls, v):
        # OSRM returns a bare [lon, lat] list
        if isinstance(v, list):
            return {"coordinates": v}
        return v


class Maneuver(BaseModel):
    bearing_after: int
    bearing_before: int
    location: Coordinate
    type: str
    modifier: Optional[str] = None
    exit: Optional[int] = None

    @field_validator("location", mode="before")
    def _parse_location(cls, v):
        if isinstance(v, list):
            return {"coordinates": v}
        return v


class Step(BaseModel):
    intersections: List[Intersection]
    driving_side: str
    geometry: str
    maneuver: Maneuver
    name: str
    mode: str
    weight: float
    duration: float
    distance: float


class Leg(BaseModel):
    steps: List[Step]


class Route(BaseModel):
    legs: List[Leg]
    weight_name: str
    weight: float
    duration: float
    distance: float
    summary: Optional[str] = ""


class Waypoint(BaseModel):
    hint: str
    location: Coordinate
    name: str
    distance: float

    @field_validator("location", mode="before")
    def _parse_location(cls, v):
        if isinstance(v, list):
            return {"coordinates": v}
        return v


class OSRMResponse(BaseModel):
    code: str
    routes: List[Route]
    waypoints: List[Waypoint]


def get_route(
    start_coordinate: Coordinate,
    end_coordinate: Coordinate,
    *,
    base_url: str = "https://routing.openstreetmap.de/routed-car/route/v1/driving"
) -> OSRMResponse:
    """
    Fetch a driving route between two points from the OSRM API and parse it.
    """
    coords = f"{start_coordinate.lon},{start_coordinate.lat};{end_coordinate.lon},{end_coordinate.lat}"
    params = {
        "overview": "full",
        "geometries": "polyline",
        "steps": "true",
    }
    headers = {
        "Accept": "*/*",
        "Accept-Language": "en-US,en;q=0.9",
    }

    resp = httpx.get(f"{base_url}/{coords}", params=params, headers=headers)
    resp.raise_for_status()
    data = resp.json()
    return OSRMResponse.model_validate(data)


def main():
    start_lat, start_lon = 4.718556, -74.044338
    end_lat, end_lon = 4.9112815, -73.9422946

    start = Coordinate(coordinates=[start_lon, start_lat])
    end = Coordinate(coordinates=[end_lon, end_lat])

    resp = get_route(start, end)
    if resp.code.lower() != "ok":
        print(f"-> API returned error code: {resp.code}")
        return

    for route_idx, route in enumerate(resp.routes, start=1):
        print(f"\nRoute {route_idx}: summary='{route.summary}'  🛣  {route.distance:.0f} m, {route.duration:.0f} s")
        for leg_idx, leg in enumerate(route.legs, start=1):
            print(f"  Leg {leg_idx}:")
            for step_idx, step in enumerate(leg.steps, start=1):
                print(f"Step location: {step.maneuver.location.to_3857().coordinates}")

if __name__ == "__main__":
    main()

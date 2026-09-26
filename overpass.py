import requests
from config import OVERPASS_API_URL

def fetch_osm_data(bbox: list[float]) -> dict:
    s, w, n, e = bbox
    query = f"""
    [out:json][timeout:25];
    (
      node({s},{w},{n},{e});
      way({s},{w},{n},{e});
      relation({s},{w},{n},{e});
    );
    out body;
    >;
    out skel qt;
    """
    response = requests.post(OVERPASS_API_URL, data={"data": query})
    if response.status_code != 200:
        raise RuntimeError(f"Overpass API failed with status {response.status_code}")
    return response.json()
    
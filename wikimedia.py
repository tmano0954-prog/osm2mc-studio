import requests
from config import WIKIMEDIA_API_URL

def get_mutcd_shield_url(shield_name: str) -> str | None:
    params = {
        "action": "query",
        "format": "json",
        "list": "search",
        "srsearch": f"file:{shield_name} MUTCD SVG",
        "srlimit": 1
    }
    res = requests.get(WIKIMEDIA_API_URL, params=params).json()
    results = res.get("query", {}).get("search", [])
    if not results:
        return None
    
    file_title = results[0]["title"]
    info_params = {
        "action": "query",
        "format": "json",
        "prop": "imageinfo",
        "titles": file_title,
        "iiprop": "url"
    }
    info_res = requests.get(WIKIMEDIA_API_URL, params=info_params).json()
    pages = info_res.get("query", {}).get("pages", {})
    for _, val in pages.items():
        if "imageinfo" in val:
            return val["imageinfo"][0]["url"]
    return None
    
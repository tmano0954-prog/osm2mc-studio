import os

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATA_ASSETS_DIR = os.path.join(BASE_DIR, "data_assets")

# Points directly to output/worlds/
OUTPUT_DIR = os.path.join(BASE_DIR, "output", "worlds")
CACHE_DIR = os.path.join(DATA_ASSETS_DIR, "cached_shields")

OVERPASS_API_URL = "https://overpass-api.de/api/interpreter"
WIKIMEDIA_API_URL = "https://commons.wikimedia.org/w/api.php"

# Automatically create output/worlds if it doesn't exist yet
os.makedirs(OUTPUT_DIR, exist_ok=True)
os.makedirs(CACHE_DIR, exist_ok=True)

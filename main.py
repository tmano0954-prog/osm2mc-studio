from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import sys
import os

sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from services.overpass import fetch_osm_data
from services.wikimedia import get_mutcd_shield_url
from services.map_art import convert_image_to_map_dat
from utils.zip_builder import create_world_zip

app = FastAPI(title="OSM2MC Studio Backend API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class GenerationRequest(BaseModel):
    bbox: list[float]  # [south, west, north, east]
    version: str
    export_format: str
    enable_sign_maps: bool = True

@app.get("/")
def read_root():
    return {"status": "online", "engine": "OSM2MC Studio v1.0.0"}

@app.post("/api/generate")
async def generate_world(req: GenerationRequest):
    try:
        osm_json = fetch_osm_data(req.bbox)
        
        if req.enable_sign_maps:
            shield_url = get_mutcd_shield_url("Florida State Road 528")
            if shield_url:
                convert_image_to_map_dat(shield_url, map_id=0)

        zip_filename = create_world_zip(req.export_format)
        
        return {
            "success": True,
            "download_url": f"/downloads/{zip_filename}",
            "summary": "1:1 Real world successfully converted to Minecraft!"
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
    
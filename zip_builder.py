import zipfile
import os
from config import OUTPUT_DIR

def create_world_zip(export_format: str) -> str:
    zip_filename = f"OSM_World_{export_format}.zip"
    zip_filepath = os.path.join(OUTPUT_DIR, zip_filename)
    
    with zipfile.ZipFile(zip_filepath, 'w', zipfile.ZIP_DEFLATED) as zipf:
        zipf.writestr("level.dat", b"Minecraft World Data Placeholder")
        zipf.writestr("region/r.0.0.mca", b"Anvil Region Data Placeholder")
    
    return zip_filename
    
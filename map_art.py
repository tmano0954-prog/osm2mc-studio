from PIL import Image
import requests
from io import BytesIO

def convert_image_to_map_dat(image_url: str, map_id: int) -> bytes:
    res = requests.get(image_url)
    img = Image.open(BytesIO(res.content)).convert("RGB")
    img = img.resize((128, 128), Image.Resampling.LANCZOS)
    return b"\x0a\x00\x00"  # Map NBT Byte Stream Placeholder
    
import utility as u
from pathlib import Path

DAYNUM = u.ints(Path(__file__).stem)[0]
data = Path(f"input/{DAYNUM}s").read_text()
data = Path(f"input/{DAYNUM}").read_text()

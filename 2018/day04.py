from prelude import *


def parse_guard_record(record):
    record = record.replace("[", "").replace("]", "")
    ymd, time, *action = record.split(" ")
    y, m, d = [int(p) for p in ymd.split("-")]
    h, mi = [int(p) for p in time.split(":")]
    action = ' '.join(action)
    if 'Guard' in action:
        action = action.split(" ")[1][1:]
    elif 'asleep' in action:
        action = 'sleep'
    elif 'wakes' in action:
        action = 'awake'
    return (y, m, d, h, mi, action)

# def day04p1(records):
#     clock_per_guard = defaultdict(lambda: [0] * 24)
#     current_guard = None
#     for record in records:
#         y, m, d, h, mi, action = record
#         if action in ['sleep', 'awake']:
#             if 
#             for hour in 
#             clock_per_guard
#         print(record)
#     pass


# if __name__ == "__main__":
#     records = sorted([parse_guard_record(r) for r in Path("input/day04").read_text().splitlines()],
#             key=lambda x: x[:5])
#     for r in records[0:10]:
#         print(r)
#     print(day04p1())

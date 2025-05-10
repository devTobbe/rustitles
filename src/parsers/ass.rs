
const ASS_PATTERN: String = r"(?m)^Dialogue:\s*(?:|\w+\W)(?:Marked=\d+,\s*)?\d+,(?P<StartTime>\d+:\d+:\d+\.\d+),(?P<EndTime>\d+:\d+:\d+\.\d+),[^,]*,[^,]*,\d+,\d+,\d+,[^,]*,(?P<Content>.+)$";

"""251: OLD against NEW on the same games by process CPU time (GetProcessTimes), in turn, K passes: CPU time
holds better than wall time on a loaded box (preemption doesn't count; cache contention still does).

    python cpu_pair.py OLD_EXE NEW_EXE PASSES -- ARGS...   (ARGS: an `aic-sim` command line, run from here)

Prints each pass's CPU seconds and the NEW / OLD ratio of the medians and of the minimums.
"""
import ctypes
import os
import statistics
import subprocess
import sys
from ctypes import wintypes

i = sys.argv.index('--')
old, new, passes = sys.argv[1], sys.argv[2], int(sys.argv[3])
args = sys.argv[i + 1:]
k32 = ctypes.windll.kernel32
k32.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4


def cpu(exe):
    p = subprocess.Popen([os.path.abspath(exe)] + args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    p.wait()
    c, e, k, u = (wintypes.FILETIME() for _ in range(4))
    if not k32.GetProcessTimes(int(p._handle), ctypes.byref(c), ctypes.byref(e), ctypes.byref(k), ctypes.byref(u)):
        raise OSError('GetProcessTimes failed')
    ft = lambda f: (f.dwHighDateTime << 32 | f.dwLowDateTime) / 1e7
    return ft(k) + ft(u)


a, b = [], []
for n in range(passes):
    order = [(old, a), (new, b)] if n % 2 == 0 else [(new, b), (old, a)]
    for exe, out in order:
        out.append(cpu(exe))
    print(f'pass {n + 1}: old {a[-1]:.2f} s, new {b[-1]:.2f} s, new/old {b[-1] / a[-1]:.3f}', flush=True)
print(f'CPU s median old {statistics.median(a):.2f} new {statistics.median(b):.2f} '
      f'ratio {statistics.median(b) / statistics.median(a):.3f}; min ratio {min(b) / min(a):.3f}')

"""325: one card scene downsized for the player view: `python scene-thumb.py SRC.png DST.jpg WIDTH`.

The dev server (vite.config.ts, `/__scene/`) runs it on a scene's first request and serves the cached JPEG after
(out/scene-cache/, git-ignored): the originals are 3.6 MB PNGs in the shared art-originals/, never committed or
copied into the repo.
"""
import os
import sys

from PIL import Image


def main():
    src, dst, width = sys.argv[1], sys.argv[2], int(sys.argv[3])
    with Image.open(src) as im:
        im = im.convert('RGB')
        if im.width > width:
            im = im.resize((width, round(im.height * width / im.width)), Image.LANCZOS)
        tmp = dst + '.part'
        im.save(tmp, 'JPEG', quality=82, optimize=True)
    os.replace(tmp, dst)


if __name__ == '__main__':
    main()

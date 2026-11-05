"""Read and write tools/card-art/scenes.json in its own layout: one scene, tag or batch row a line.

`json.dump` would rewrite the whole file; `dump` reproduces it byte for byte (load then dump of
an unedited file gives the same bytes: `python tools/card-art/scenes_io.py check`).

    import scenes_io as sio           # with tools/card-art on sys.path
    s = sio.load()
    sio.release(s, 'plant_overdrive')  # the card's scene is suggested for no card now
    sio.assign(s, 'fabrication_megaplex', 'foundry_furnace_hall')
    sio.save(s)

From the shell: `python tools/card-art/scenes_io.py assign CARD SCENE`, `... release CARD`.
"""
import json
import sys
from pathlib import Path

SCENES = Path(__file__).resolve().parent / 'scenes.json'


def load(path=SCENES):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def _inner(d):
    if isinstance(d, dict):
        return '{ ' + ', '.join(f'{json.dumps(k)}: {json.dumps(v, ensure_ascii=False)}' for k, v in d.items()) + ' }'
    return json.dumps(d, ensure_ascii=False)


def dump(obj):
    """The file's text: top-level keys a line each, `scenes` a line per scene under its tag,
    `tags` and `batch` a line per row."""
    lines = ['{']
    keys = list(obj)
    for i, k in enumerate(keys):
        comma = ',' if i < len(keys) - 1 else ''
        v = obj[k]
        if k == 'scenes':
            lines.append('  "scenes": {')
            tags = list(v)
            for j, t in enumerate(tags):
                lines.append(f'    {json.dumps(t)}: {{')
                ids = list(v[t])
                for m, sid in enumerate(ids):
                    lines.append(f'      {json.dumps(sid)}: {_inner(v[t][sid])}' + (',' if m < len(ids) - 1 else ''))
                lines.append('    }' + (',' if j < len(tags) - 1 else ''))
            lines.append('  }' + comma)
        elif k in ('tags', 'batch'):
            lines.append(f'  {json.dumps(k)}: {{')
            ids = list(v)
            for m, sid in enumerate(ids):
                lines.append(f'    {json.dumps(sid)}: {_inner(v[sid])}' + (',' if m < len(ids) - 1 else ''))
            lines.append('  }' + comma)
        else:
            lines.append(f'  {json.dumps(k)}: {json.dumps(v, ensure_ascii=False)}' + comma)
    lines.append('}')
    return '\n'.join(lines) + '\n'


def save(obj, path=SCENES):
    Path(path).write_text(dump(obj), encoding='utf-8', newline='\n')


def find(s, scene):
    """The tag group holding scene id `scene`."""
    tag = next((t for t, g in s['scenes'].items() if scene in g), None)
    if tag is None:
        raise KeyError(f'no scene {scene}')
    return tag


def scene_of(s, card):
    """(tag, scene id) of the scene suggested for `card`, or None."""
    return next(((t, sid) for t, g in s['scenes'].items() for sid, sc in g.items() if sc.get('card') == card), None)


def release(s, card):
    """Suggest `card`'s scene for no card: `card` goes, `was` leads (a reused scene keeps its
    first `was`). Returns the scene id, or None if the card had none."""
    at = scene_of(s, card)
    if at is None:
        return None
    tag, sid = at
    sc = dict(s['scenes'][tag][sid])
    del sc['card']
    s['scenes'][tag][sid] = {'was': sc.pop('was', card), **sc}
    return sid


def assign(s, card, scene):
    """Suggest `scene` for `card`, releasing the card's current scene first; the scene must be
    suggested for no card. `card` leads, any `was` is kept (written for `was`, reused on `card`).
    Returns the released scene id, or None."""
    tag = find(s, scene)
    held = s['scenes'][tag][scene].get('card')
    if held is not None and held != card:
        raise ValueError(f'{scene} is suggested for {held}: release it first')
    old = release(s, card)
    sc = s['scenes'][tag][scene]
    s['scenes'][tag][scene] = {'card': card, **sc}
    return old


def main(argv):
    if argv == ['check']:
        text = SCENES.read_text(encoding='utf-8')
        ok = dump(load()) == text
        print('scenes.json round-trips byte for byte' if ok else 'scenes.json does NOT round-trip: dump differs')
        return 0 if ok else 1
    s = load()
    if len(argv) == 3 and argv[0] == 'assign':
        old = assign(s, argv[1], argv[2])
        print(f'{argv[1]}: {old or "(no scene)"} -> {argv[2]}')
    elif len(argv) == 2 and argv[0] == 'release':
        print(f'{argv[1]}: released {release(s, argv[1]) or "(no scene)"}')
    else:
        sys.exit(__doc__)
    save(s)
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

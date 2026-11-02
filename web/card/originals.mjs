// Where the art originals live: Node only, no dependencies, so the card viewer's server can
// read them with no npm install. See tools/card-art/README.md.
import { existsSync } from 'node:fs';
import { join } from 'node:path';

const DRIVE_ORIGINALS = 'C:\\Code\\gdrive-sync\\aicomplete\\art-originals';

/** AIC_ART_ORIGINALS, else the junction when `repo` has one, else the Drive folder it points at. */
export function originalsDir(repo) {
  const junction = join(repo, 'art-originals');
  return process.env.AIC_ART_ORIGINALS ?? (existsSync(junction) ? junction : DRIVE_ORIGINALS);
}

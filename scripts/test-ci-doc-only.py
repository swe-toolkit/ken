#!/usr/bin/env python3
import subprocess
subprocess.run(['python3','scripts/ci-doc-only-topology.py','check'],check=True)
print('generated topology check passed')

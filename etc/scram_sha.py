#!/usr/bin/env nix-shell
#! nix-shell -i python3 -p "python3.withPackages (ps: [ps.scramp])"
import scramp, sys, base64

_, passwd = sys.argv
m = scramp.ScramMechanism()
salt, x, y, n = m.make_auth_info(passwd)
print(f"""
SCRAM-SHA256${n}:
{base64.b64encode(salt).decode()}${base64.b64encode(x).decode()}:
{base64.b64encode(y).decode()}
""")

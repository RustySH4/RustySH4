
import json

# Opening JSON file
f = open('godbolt_compile.json')

data = json.load(f)
res = "let program = vec!["

for asm in data["asm"]:
    if not asm.get("opcodes"):
        continue

    for opcode in asm.get("opcodes"):
        res += "0x" + opcode + ", "

res += "];"
print(res)
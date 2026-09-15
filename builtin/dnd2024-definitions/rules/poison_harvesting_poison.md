---
splat_schema: 1
splat_id: 83fba512-5429-5d13-a438-117acd034f20
splat_kind: rule
splat_key: dnd2024:rules/poison_harvesting_poison
splat_name: Harvesting Poison
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_harvesting_poison
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Poison
  line_span:
    first: 9807
    count: 12
splat_clauses:
- key: source_rules
  text: '# Harvesting Poison A character can attempt to harvest poison from a venomous creature that is
    dead or has the Incapacitated condition. The effort takes 1d6 minutes, after which the character makes
    a DC 20 Intelligence (Nature) check using a Poisoner’s Kit. On a successful check, the character harvests
    enough poison for a single dose, and no additional poison can be harvested from that creature. On
    a failed check, the character is unable to extract any poison. If the character fails the check by
    5 or more, the character is subjected to the creature’s poison.'
  sha256: e3c86585531951511775a889ec7adfb6f86ba6153f8e0f47a47995feb4a8ac4e
  status: needs_semantic_conversion
---

# Harvesting Poison

<!-- SRD page 197 -->

A character can attempt to harvest poison from a\
venomous creature that is dead or has the Incapaci-\
tated condition. The effort takes 1d6 minutes, after\
which the character makes a DC 20 Intelligence\
(Nature) check using a Poisoner’s Kit. On a success-\
ful check, the character harvests enough poison\
for a single dose, and no additional poison can be\
harvested from that creature. On a failed check,\
the character is unable to extract any poison. If the\
character fails the check by 5 or more, the character\
is subjected to the creature’s poison.\

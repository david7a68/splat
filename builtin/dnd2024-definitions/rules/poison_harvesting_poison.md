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
  parameters:
    target_requirements:
      venomous: true
      any_state:
      - dead
      - incapacitated
      already_harvested: false
    time:
      dice:
        count: 1
        sides: 6
      unit: minute
    required_tool: poisoners_kit
    check:
      ability: intelligence
      skill: nature
      dc: 20
    success:
      poison_doses: 1
      target_further_harvest_prohibited: true
    failure:
      poison_doses: 0
    failure_margin_at_least_5:
      subjected_to: target_poison
  implementation: dnd2024:rules_poison_harvesting_poison
splat_contracts:
- id: dnd2024:rules_poison_harvesting_poison
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
    kit:
      type: item_instance
      required: true
  outputs:
  - poison_doses
  - poison_exposure
  - harvest_state
  events: []
  capabilities:
  - poison
  - crafting
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: &id001
  - dnd2024:rules_poison_harvesting_poison
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: e045b5c13e3befe689bf7ccb1ac7fdd8441e3ecca981c06df443a303af7453ec
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
  status: contract_specified
splat_requires_mechanics: *id001
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

---
splat_schema: 1
splat_id: 5a6d8845-22a6-51bb-9166-0feb2dca92e0
splat_kind: rule
splat_key: dnd2024:rules/poison_oil_of_taggit_400_gp
splat_name: Oil of Taggit (400 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_oil_of_taggit_400_gp
  parameters:
    poison:
      type: contact
      cost_gp: 400
    save:
      ability: constitution
      dc: 13
    failure:
      condition:
        id: poisoned
        duration:
          value: 24
          unit: hours
      unconscious_while_poisoned: true
    wake_triggers:
    - damage_taken
    wake_ends_poisoned: false
  implementation: dnd2024:rules_poison_oil_of_taggit_400_gp
splat_contracts:
- id: dnd2024:rules_poison_oil_of_taggit_400_gp
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events: []
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rules_poison_oil_of_taggit_400_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 50202cae76ec0d19bce713e55f2f8353375d68ae6e25b3cd1469bebb24bea95f
  parent_name: Poison
  line_span:
    first: 9875
    count: 7
splat_clauses:
- key: source_rules
  text: '# Oil of Taggit (400 GP) Contact Poison A creature subjected to Oil of Taggit must succeed on
    a DC 13 Constitution saving throw or have the Poisoned condition for 24 hours. The creature also has
    the Unconscious condition while Poisoned in this way. It wakes up if it takes damage.'
  sha256: b00631d46623a4a1a4c9fad064380dff18332da66a0c467ba3df75eac4cb7592
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oil of Taggit (400 GP)

<!-- SRD page 198 -->

Contact Poison\
A creature subjected to Oil of Taggit must succeed\
on a DC 13 Constitution saving throw or have the\
Poisoned condition for 24 hours. The creature also\
has the Unconscious condition while Poisoned in\
this way. It wakes up if it takes damage.\

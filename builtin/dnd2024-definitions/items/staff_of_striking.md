---
splat_schema: 1
splat_id: 6681752d-18a3-575e-9ad6-34b2d6c70531
splat_kind: item
splat_key: dnd2024:items/staff_of_striking
splat_name: Staff of Striking
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_striking
splat_sources:
- title: SRD 5.2.1
  locator: pp. 246
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/staff_of_striking
  implementation: dnd2024:item_staff_of_striking
  parameters:
    weapon:
      key: dnd2024:equipment/quarterstaff
    magical_weapon: true
    attack_bonus: 3
    damage_bonus: 3
    charges:
      initial: 10
      maximum: 10
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 6
            add: 4
        cap_at_maximum: true
      last_charge:
        roll:
          roll:
            count: 1
            sides: 20
        'on': 1
        result: nonmagical_quarterstaff
    on_melee_hit:
      optional_charge_spend:
        minimum: 1
        maximum: 3
      extra_damage:
        type: force
        amount:
          roll:
            count: charges_spent
            sides: 6
    attunement_required: true
splat_contracts:
- id: dnd2024:item_staff_of_striking
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - attack.hit
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_charges
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: ad69dc1569ba56cecd9329605d74657f623ab3516ce74f27e2839db377309df5
  source_path: builtin/srd-5.2.1/magic_item/staff_of_striking.md
  pages:
  - 246
  engine_gaps: &id001
  - dnd2024:item_staff_of_striking
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: f1beb3b869ecf6402669ee1f5aec9fa11b56aaa39fba2808187493ff430c7b67
splat_clauses:
- key: source_rules
  text: '# Staff of Striking Staff, Very Rare (Requires Attunement) This staff can be wielded as a magic
    Quarterstaff that grants a +3 bonus to attack rolls and damage rolls made with it. The staff has 10
    charges. When you hit with a melee attack using it, you can expend up to 3 charges. For each charge
    you expend, the target takes an extra 1d6 Force damage. Regaining Charges. The staff regains 1d6 +
    4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff becomes
    a nonmagical Quarterstaff.'
  sha256: 6d860c1231b4e4daee0b1f8e78327941296cf13a568137b6cc21e5bb5a11c9cd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Striking

<!-- SRD page 246 -->

Staff, Very Rare (Requires Attunement)\
This staff can be wielded as a magic Quarterstaff\
that grants a +3 bonus to attack rolls and damage\
rolls made with it.\
The staff has 10 charges. When you hit with\
a melee attack using it, you can expend up to 3\
charges. For each charge you expend, the target\
takes an extra 1d6 Force damage.\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff becomes a\
nonmagical Quarterstaff.\

Original layout: [[../../srd-5.2.1/pages/246|page 246]].

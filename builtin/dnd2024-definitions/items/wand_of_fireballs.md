---
splat_schema: 1
splat_id: aee61ca4-d6e7-5870-9535-bfa3fdbf2dd2
splat_kind: item
splat_key: dnd2024:items/wand_of_fireballs
splat_name: Wand of Fireballs
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_fireballs
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250, 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  stackable: false
  equipment:
    allowed:
    - carried
    - wielded
    attunement: required
    attunement_requirement: spellcaster
splat_resources:
  charges:
    name: Charges
    max: 7
    initial: 7
    recovery:
    - key: dawn
      mechanic: dnd2024:dawn
      subject: owner
      roll:
        key: recovery
        expression: 1d6 + 1
        scope: use
      restore:
        add:
          roll: recovery
splat_actions:
  cast:
    name: Cast Fireball
    activation:
      economy: action
      category: magic
      timing: on_turn
      cost_at: commit
    steps:
    - key: cast
      operation: dnd2024:wand_of_fireballs_2024
      spell:
        key: dnd2024:spells/fireball
      parameters:
        requires:
        - held
        - attuned
        charges_min: 1
        charges_max: 3
        base_spell_level: 3
        level_per_extra_charge: 1
        save_dc: 15
        spell_slot_cost: 0
        last_charge:
          roll: 1d20
          destroy_on:
          - 1
          destroyed_form: ashes
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6b9828133e84bcf63e3668f7771dd059473117f97116c0f4f14e41084d9450b2
  source_path: builtin/srd-5.2.1/magic_item/wand_of_fireballs.md
  pages:
  - 250
  - 251
  engine_gaps: &id001
  - dnd2024:wand_of_fireballs_2024
  conversion_gaps: []
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Wand of Fireballs Wand, Rare (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can expend no more than 3 charges to cast Fireball (save DC 15) from it. For
    1 charge, you cast the level 3 version of the spell. You can increase the spell’s level by 1 for each
    additional charge you expend. Regaining Charges. The wand regains 1d6 + 1 expended charges daily at
    dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is
    destroyed.'
  sha256: b2ceb6636047765b7c231fc476ef7bed6f3cd16c4c2b19667ee55ece745aaae5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Fireballs

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement by a Spellcaster)\
This wand has 7 charges. While holding it, you can\
expend no more than 3 charges to cast Fireball (save\
DC 15) from it. For 1 charge, you cast the level 3 ver-\
sion of the spell. You can increase the spell’s level by\
1 for each additional charge you expend.\

<!-- SRD page 251 -->

Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]], [[../../srd-5.2.1/pages/251|page 251]].

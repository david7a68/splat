---
splat_schema: 1
splat_id: e42200ee-5fd6-5f34-aa99-1f3b4d1a5516
splat_kind: item
splat_key: dnd2024:items/potion_of_invisibility
splat_name: Potion of Invisibility
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_invisibility
  implementation: dnd2024:item_potion_of_invisibility
  parameters:
    trigger: potion_drunk
    consumed: true
    effect:
      condition: invisible
      duration:
        amount: 1
        unit: hours
      ends_on:
      - user.attack_roll
      - user.damage_dealt
      - user.spell_cast
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_invisibility
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
  - item.consumed
  - attack.rolled
  - damage.resolved
  - spell.cast_completed
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - effect_duration
  - damage
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2a5e2dcc5fa4f5b6d5581388ce7773f580d8314153bf92af07563f504b9dab40
  source_path: builtin/srd-5.2.1/magic_item/potion_of_invisibility.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 2f4b9f96063d2de21363e89c09bdc01a5df453bab3519f4225acb26808aac2ce
splat_clauses:
- key: source_rules
  text: '# Potion of Invisibility Potion, Rare This potion’s container looks empty but feels as though
    it holds liquid. When you drink the potion, you have the Invisible condition for 1 hour. The effect
    ends early if you make an attack roll, deal damage, or cast a spell.'
  sha256: 6d58c00ba8306d556ee1945f0ffcb9a44bca6aa365c89f0e382d4b044789ddc0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Invisibility

<!-- SRD page 236 -->

Potion, Rare\
This potion’s container looks empty but feels as\
though it holds liquid. When you drink the potion,\
you have the Invisible condition for 1 hour. The\
effect ends early if you make an attack roll, deal\
damage, or cast a spell.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

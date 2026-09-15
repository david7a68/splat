---
splat_schema: 1
splat_id: 47a973dc-d6b6-543c-833a-482239b19b4e
splat_kind: item
splat_key: dnd2024:items/demon_armor
splat_name: Demon Armor
splat_origin:
  key: dnd2024:reference/magic_item/demon_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/demon_armor
  implementation: dnd2024:item_demon_armor
  parameters:
    equipment_state: worn
    armor_class_bonus: 1
    languages:
    - abyssal
    unarmed_strike:
      damage_dice:
        dice: 1
        sides: 8
        bonus: 0
      damage_type: slashing
      replaces: normal_bludgeoning_damage
      attack_bonus: 1
      damage_bonus: 1
    curse:
      starts: don_armor
      cannot_doff: true
      unlock:
      - spell:
          key: dnd2024:spells/remove_curse
      - magic: similar_curse_removal
      attack_disadvantage_against: demons
      save_disadvantage_against:
        source: demons
        effect:
        - spells
        - special_abilities
      attunement_required: false
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_demon_armor
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    event:
      required: false
      type: event
  outputs:
  - modifiers
  - curse_state
  - effects
  events:
  - equipment.state_changed
  - attunement.completed
  - damage.received
  - spell.cast_completed
  capabilities:
  - curses
  - equipment
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6ec9f92f63a19c60568eace20d9e3b70b8820496371ea8746d6305df836374ef
  source_path: builtin/srd-5.2.1/magic_item/demon_armor.md
  pages:
  - 218
  engine_gaps: &id001
  - dnd2024:item_demon_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 4d5d8c2fc8e799ac5b5e725156651961c86e2261d718c1ff07856f382314e115
splat_clauses:
- key: source_rules
  text: '# Demon Armor Armor (Any Light, Medium, or Heavy), Very Rare (Requires Attunement) While wearing
    this armor, you gain a +1 bonus to Armor Class, and you know Abyssal. In addition, the armor’s clawed
    gauntlets allow your Unarmed Strikes to deal 1d8 Slashing damage instead of the usual Bludgeoning
    damage, and you gain a +1 bonus to the attack and damage rolls of your Unarmed Strikes. Curse. Once
    you don this cursed armor, you can’t doff it unless you are targeted by a Remove Curse spell or similar
    magic. While wearing the armor, you have Disadvantage on attack rolls against demons and on saving
    throws against their spells and special abilities.'
  sha256: c147eab6de3f1a94660c98e4daf76d3b54393c11c3c008ed5f68797262dcc4c5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Demon Armor

<!-- SRD page 218 -->

Armor (Any Light, Medium, or Heavy), Very Rare\
(Requires Attunement)\
While wearing this armor, you gain a +1 bonus to\
Armor Class, and you know Abyssal. In addition,\
the armor’s clawed gauntlets allow your Unarmed\
Strikes to deal 1d8 Slashing damage instead of the\
usual Bludgeoning damage, and you gain a +1 bonus\
to the attack and damage rolls of your Unarmed\
Strikes.\
Curse. Once you don this cursed armor, you can’t\
doff it unless you are targeted by a Remove Curse\
spell or similar magic. While wearing the armor,\
you have Disadvantage on attack rolls against de-\
mons and on saving throws against their spells and\
special abilities.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].

---
splat_schema: 1
splat_id: 21c77eab-aa16-5c1f-9f60-33cb56973ffb
splat_kind: item
splat_key: dnd2024:items/shield_of_missile_attraction
splat_name: Shield of Missile Attraction
splat_origin:
  key: dnd2024:reference/magic_item/shield_of_missile_attraction
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/shield_of_missile_attraction
  implementation: dnd2024:item_shield_of_missile_attraction
  parameters:
    while_held:
      damage_resistance:
        source: attacks_with_ranged_weapons
    curse:
      apply_on: attunement_completed
      persists_after_doff: true
      ends_on:
      - spell:
          key: dnd2024:spells/remove_curse
        target: user
      - magic_equivalent: remove_curse
        adjudicator: gm
      redirect_attack:
        weapon_category: ranged
        original_target_distance_from_user_ft_max: 10
        new_target: user
    attunement_required: true
splat_contracts:
- id: dnd2024:item_shield_of_missile_attraction
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
  - attack.target_selected
  - damage.pending
  - item.attunement_completed
  - spell.cast_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
  - curses
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d968fd797797ca057f7481fa1789a38d97f59884bac4d57ed4e0d5c78c699dab
  source_path: builtin/srd-5.2.1/magic_item/shield_of_missile_attraction.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_shield_of_missile_attraction
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 8cbac690b9f5cb161ca399c1a2375745d65a1ce7200c4f80f478c682e0341be0
splat_clauses:
- key: source_rules
  text: '# Shield of Missile Attraction Armor (Shield), Rare (Requires Attunement) While holding this
    Shield, you have Resistance to damage from attacks made with Ranged weapons. Curse. This Shield is
    cursed. Attuning to it curses you until you are targeted by a Remove Curse spell or similar magic.
    Removing the Shield fails to end the curse on you. Whenever an attack with a Ranged weapon targets
    a creature within 10 feet of you, the curse causes you to become the target instead.'
  sha256: 7c0b09a76f492048ae5ff9b11e01fcf5196b56833214aaca0a7855080a5a216d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shield of Missile Attraction

<!-- SRD page 243 -->

Armor (Shield), Rare (Requires Attunement)\
While holding this Shield, you have Resistance to\
damage from attacks made with Ranged weapons.\
Curse. This Shield is cursed. Attuning to it curses\
you until you are targeted by a Remove Curse spell\
or similar magic. Removing the Shield fails to\
end the curse on you. Whenever an attack with a\
Ranged weapon targets a creature within 10 feet\
of you, the curse causes you to become the target\
instead.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

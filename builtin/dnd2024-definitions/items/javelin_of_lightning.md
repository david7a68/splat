---
splat_schema: 1
splat_id: ecdd6fe2-79ad-5b35-9058-b2174a47ca71
splat_kind: item
splat_key: dnd2024:items/javelin_of_lightning
splat_name: Javelin of Lightning
splat_origin:
  key: dnd2024:reference/magic_item/javelin_of_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/javelin_of_lightning
  implementation: dnd2024:item_javelin_of_lightning
  parameters:
    on_hit_damage_type_choice:
    - piercing
    - lightning
    lightning_bolt:
      trigger: throw_at_target
      max_target_range_ft: 120
      replaces: ranged_attack_roll
      area:
        shape: line
        width_ft: 5
        from: user
        to: target
      targets:
      - target
      - all_other_creatures_in_line
      exclude:
      - user
      save:
        ability: dexterity
        dc: 13
        failure:
          damage:
            amount:
              roll:
                count: 4
                sides: 6
            type: lightning
        success:
          damage_fraction:
          - 1
          - 2
      after_damage: return_weapon_to_hand
      uses:
        maximum: 1
        recovery_event: time.dawn
    attunement_required: false
splat_contracts:
- id: dnd2024:item_javelin_of_lightning
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
  - attack.pending
  - time.dawn
  capabilities:
  - item_effects
  - dice
  - spatial_targeting
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 449f6b6d02d85166767c67b4cfe61cfce9f8a3029c2b868c496fdebcf92c0823
  source_path: builtin/srd-5.2.1/magic_item/javelin_of_lightning.md
  pages:
  - 228
  engine_gaps: &id001
  - dnd2024:item_javelin_of_lightning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: a4a261f05acdb67bbebc20818f6914545076987f513a0350f1d09787eb1f6d8e
splat_clauses:
- key: source_rules
  text: '# Javelin of Lightning Weapon (Javelin), Uncommon Each time you make an attack roll with this
    magic weapon and hit, you can have it deal Lightning damage instead of Piercing damage. Lightning
    Bolt. When you throw this weapon at a target no farther than 120 feet from you, you can forgo making
    a ranged attack roll and instead turn the weapon into a bolt of lightning. This bolt forms a 5-foot-wide
    Line between you and the target. The target and each other creature in the Line (excluding you) makes
    a DC 13 Dexterity saving throw, taking 4d6 Lightning damage on a failed save or half as much damage
    on a successful one. Immediately after dealing this damage, the weapon reappears in your hand. This
    property can’t be used again until the next dawn.'
  sha256: cc8c1b71bb3f8280b7e6437b405716ebb7a342acca6a2986ad779bf08043d15a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Javelin of Lightning

<!-- SRD page 228 -->

Weapon (Javelin), Uncommon\
Each time you make an attack roll with this magic\
weapon and hit, you can have it deal Lightning dam-\
age instead of Piercing damage.\
Lightning Bolt. When you throw this weapon at\
a target no farther than 120 feet from you, you can\
forgo making a ranged attack roll and instead turn\
the weapon into a bolt of lightning. This bolt forms\
a 5-foot-wide Line between you and the target. The\
target and each other creature in the Line (exclud-\
ing you) makes a DC 13 Dexterity saving throw, tak-\
ing 4d6 Lightning damage on a failed save or half\
as much damage on a successful one. Immediately\
after dealing this damage, the weapon reappears in\
your hand. This property can’t be used again until\
the next dawn.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]].

---
splat_schema: 1
splat_id: 07e6945f-0ea0-58cd-84e9-59c41d125c5a
splat_kind: item
splat_key: dnd2024:items/ioun_stone
splat_name: Ioun Stone
splat_origin:
  key: dnd2024:reference/magic_item/ioun_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227, 228
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rarity_varies
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ioun_stone
  implementation: dnd2024:item_ioun_stone
  parameters:
    activate:
      action: magic
      toss_in_air: true
      orbit_distance_ft:
        roll:
          count: 1
          sides: 3
      maximum_simultaneously_orbiting: 3
    benefits_require_orbit: true
    orbit:
      counts_as: worn
      avoids_collisions: true
      other_creatures_cannot:
      - attack
      - snatch
    stow:
      action: utilize
      quantity: any_number_orbiting
    attunement_end:
      orbiting: fall_as_dropped
    variants:
    - name: absorption
      rarity: very_rare
      reaction:
        trigger: visible_creature_casts_spell
        max_spell_level: 4
        effect: cancel_spell
        resources_wasted: true
        capacity_spell_levels: 20
        cumulative_levels_at_capacity:
        - burn_out
        - nonmagical
        color_after: dull_gray
    - name: greater_absorption
      rarity: legendary
      reaction:
        trigger: visible_creature_casts_spell
        max_spell_level: 8
        effect: cancel_spell
        resources_wasted: true
        capacity_spell_levels: 20
        cumulative_levels_at_capacity:
        - burn_out
        - nonmagical
        color_after: dull_gray
    - name: agility
      rarity: very_rare
      ability: dexterity
      increase: 2
      maximum: 20
    - name: fortitude
      rarity: very_rare
      ability: constitution
      increase: 2
      maximum: 20
    - name: insight
      rarity: very_rare
      ability: wisdom
      increase: 2
      maximum: 20
    - name: intellect
      rarity: very_rare
      ability: intelligence
      increase: 2
      maximum: 20
    - name: leadership
      rarity: very_rare
      ability: charisma
      increase: 2
      maximum: 20
    - name: strength
      rarity: very_rare
      ability: strength
      increase: 2
      maximum: 20
    - name: awareness
      rarity: rare
      advantage:
      - initiative
      - wisdom_perception
    - name: mastery
      rarity: legendary
      proficiency_bonus_increase: 1
    - name: protection
      rarity: rare
      ac_bonus: 1
    - name: regeneration
      rarity: legendary
      healing:
        hp: 15
        interval:
          amount: 1
          unit: hours
        requires_hp_at_least: 1
    - name: reserve
      rarity: rare
      spell_storage:
        capacity_spell_levels: 4
        initial_spell_levels:
          roll:
            count: 1
            sides: 4
        initial_spells_choice: gm
        store:
          caster: any_creature
          touch_required_during_cast: true
          spell_level_min: 1
          spell_level_max: 4
          occupancy: cast_slot_level
          normal_spell_effect: false
          insufficient_capacity: expend_without_effect
        release:
          cast_as: user
          copy_original_caster:
          - slot_level
          - save_dc
          - spell_attack_bonus
          - spellcasting_ability
          remove_spell_from_storage: true
          free_capacity: true
    - name: sustenance
      rarity: rare
      needs_food: false
      needs_drink: false
    absorption_remaining_budget_policy:
      adjudicator: gm
      scope: spell_level_exceeds_remaining_lifetime_budget
      options:
      - reject_absorption
      - absorb_and_burn_out
      source_unspecified: true
    absorption_burnout_threshold:
      comparison: greater_than_or_equal
      levels: 20
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ioun_stone
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
  - item.activation_requested
  - item.attunement_ended
  - spell.cast_pending
  - time.elapsed
  - item.attunement_completed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - inventory_containers
  - item_variants
  - healing
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f3ef6e9b7756e94c77c4d32efb911a422e66b75d06ae7357dcb979fb0b0c9ad6
  source_path: builtin/srd-5.2.1/magic_item/ioun_stone.md
  pages:
  - 227
  - 228
  engine_gaps: &id001
  - dnd2024:item_ioun_stone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 3d6dc0f82f22985adb5e5b3608dd8cd8f6d6f24040c58f2bd337d51f8bedc2d2
splat_clauses:
- key: source_rules
  text: '# Ioun Stone Wondrous Item, Rarity Varies (Requires Attunement) Roughly marble sized, Ioun Stones
    are named after Ioun, a god of knowledge and prophecy revered on some worlds. Many types of Ioun Stones
    exist, each type a distinct combination of shape and color. When you take a Magic action to toss an
    Ioun Stone into the air, the stone orbits your head at a distance of 1d3 feet, conferring its benefit
    to you while doing so. You can have up to three Ioun Stones orbiting your head at the same time. Each
    Ioun Stone orbiting your head is considered to be an object you are wearing. The orbiting stone avoids
    contact with other creatures and objects, adjusting its orbit to avoid collisions and thwarting all
    attempts by other creatures to attack or snatch it. As a Utilize action, you can seize and stow any
    number of Ioun Stones orbiting your head. If your Attunement to an Ioun Stone ends while it’s orbiting
    your head, the stone falls as though you had dropped it. The type of stone determines its rarity and
    effects. Absorption (Very Rare). While this pale lavender ellipsoid orbits your head, you can take
    a Reaction to cancel a spell of level 4 or lower cast by a creature you can see. A canceled spell
    has no effect, and any resources used to cast it are wasted. Once the stone has canceled 20 levels
    of spells, it burns out, turns dull gray, and loses its magic. Agility (Very Rare). Your Dexterity
    increases by 2, to a maximum of 20, while this deep-red sphere orbits your head. Awareness (Rare).
    While this dark-blue rhomboid orbits your head, you have Advantage on Initiative rolls and Wisdom
    (Perception) checks. Fortitude (Very Rare). Your Constitution increases by 2, to a maximum of 20,
    while this pink rhomboid orbits your head. Greater Absorption (Legendary). While this marbled lavender
    and green ellipsoid orbits your head, you can take a Reaction to cancel a spell of level 8 or lower
    cast by a creature you can see. A canceled spell has no effect, and any resources used to cast it
    are wasted. Once the stone has canceled 20 levels of spells, it burns out, turns dull gray, and loses
    its magic. Insight (Very Rare). Your Wisdom increases by 2, to a maximum of 20, while this incandescent
    blue sphere orbits your head. Intellect (Very Rare). Your Intelligence increases by 2, to a maximum
    of 20, while this marbled scarlet and blue sphere orbits your head. Leadership (Very Rare). Your Charisma
    increases by 2, to a maximum of 20, while this marbled pink and green sphere orbits your head. Mastery
    (Legendary). Your Proficiency Bonus increases by 1 while this pale green prism orbits your head. Protection
    (Rare). You gain a +1 bonus to Armor Class while this dusty-rose prism orbits your head. Regeneration
    (Legendary). You regain 15 Hit Points at the end of each hour this pearly white spindle orbits your
    head if you have at least 1 Hit Point. Reserve (Rare). This vibrant purple prism stores spells cast
    into it, holding them until you use them. The stone can store up to 4 levels of spells at a time.
    When found, it contains 1d4 levels of stored spells chosen by the GM. Any creature can cast a spell
    of level 1 through 4 into the stone by touching it as the spell is cast. The spell has no effect,
    other than to be stored in the stone. If the stone can’t hold the spell, the spell is expended without
    effect. The level of the slot used to cast the spell determines how much space it uses. While this
    stone orbits your head, you can cast any spell stored in it. The spell uses the slot level, spell
    save DC, spell attack bonus, and spellcasting ability of the original caster but is otherwise treated
    as if you cast the spell. The spell cast from the stone is no longer stored in it, freeing up space.
    Strength (Very Rare). Your Strength increases by 2, to a maximum of 20, while this pale blue rhomboid
    orbits your head. Sustenance (Rare). You don’t need to eat or drink while this clear spindle orbits
    your head.'
  sha256: fe221ad033818b8e9aee0997a86d26f6905d0dfad3a401345fe275c739c09591
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ioun Stone

<!-- SRD page 227 -->

Wondrous Item, Rarity Varies (Requires Attunement)\
Roughly marble sized, Ioun Stones are named after\
Ioun, a god of knowledge and prophecy revered on\
some worlds. Many types of Ioun Stones exist, each\
type a distinct combination of shape and color.\
When you take a Magic action to toss an Ioun\
Stone into the air, the stone orbits your head at a\
distance of 1d3 feet, conferring its benefit to you\
while doing so. You can have up to three Ioun Stones\
orbiting your head at the same time.\
Each Ioun Stone orbiting your head is considered\
to be an object you are wearing. The orbiting stone\
avoids contact with other creatures and objects, ad-\
justing its orbit to avoid collisions and thwarting all\
attempts by other creatures to attack or snatch it.\
As a Utilize action, you can seize and stow any\
number of Ioun Stones orbiting your head. If your\
Attunement to an Ioun Stone ends while it’s orbit-\
ing your head, the stone falls as though you had\
dropped it.\
The type of stone determines its rarity and\
effects.\
Absorption (Very Rare). While this pale lavender\
ellipsoid orbits your head, you can take a Reaction\
to cancel a spell of level 4 or lower cast by a crea-\
ture you can see. A canceled spell has no effect, and\
any resources used to cast it are wasted. Once the\
stone has canceled 20 levels of spells, it burns out,\
turns dull gray, and loses its magic.\
Agility (Very Rare). Your Dexterity increases by\
2, to a maximum of 20, while this deep-red sphere\
orbits your head.\
Awareness (Rare). While this dark-blue rhomboid\
orbits your head, you have Advantage on Initiative\
rolls and Wisdom (Perception) checks.\
Fortitude (Very Rare). Your Constitution in-\
creases by 2, to a maximum of 20, while this pink\
rhomboid orbits your head.\
Greater Absorption (Legendary). While this mar-\
bled lavender and green ellipsoid orbits your head,\
you can take a Reaction to cancel a spell of level 8\
or lower cast by a creature you can see. A canceled\
spell has no effect, and any resources used to cast\
it are wasted. Once the stone has canceled 20 lev-\
els of spells, it burns out, turns dull gray, and loses\
its magic.\
Insight (Very Rare). Your Wisdom increases by\
2, to a maximum of 20, while this incandescent blue\
sphere orbits your head.\
Intellect (Very Rare). Your Intelligence increases\
by 2, to a maximum of 20, while this marbled scarlet\
and blue sphere orbits your head.\
Leadership (Very Rare). Your Charisma increases\
by 2, to a maximum of 20, while this marbled pink\
and green sphere orbits your head.\

<!-- SRD page 228 -->

Mastery (Legendary). Your Proficiency Bonus in-\
creases by 1 while this pale green prism orbits your\
head.\
Protection (Rare). You gain a +1 bonus to Armor\
Class while this dusty-rose prism orbits your head.\
Regeneration (Legendary). You regain 15 Hit\
Points at the end of each hour this pearly white\
spindle orbits your head if you have at least 1 Hit\
Point.\
Reserve (Rare). This vibrant purple prism stores\
spells cast into it, holding them until you use them.\
The stone can store up to 4 levels of spells at a time.\
When found, it contains 1d4 levels of stored spells\
chosen by the GM.\
Any creature can cast a spell of level 1 through 4\
into the stone by touching it as the spell is cast. The\
spell has no effect, other than to be stored in the\
stone. If the stone can’t hold the spell, the spell is\
expended without effect. The level of the slot used\
to cast the spell determines how much space it uses.\
While this stone orbits your head, you can cast\
any spell stored in it. The spell uses the slot level,\
spell save DC, spell attack bonus, and spellcast-\
ing ability of the original caster but is otherwise\
treated as if you cast the spell. The spell cast from\
the stone is no longer stored in it, freeing up space.\
Strength (Very Rare). Your Strength increases by\
2, to a maximum of 20, while this pale blue rhom-\
boid orbits your head.\
Sustenance (Rare). You don’t need to eat or drink\
while this clear spindle orbits your head.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]], [[../../srd-5.2.1/pages/228|page 228]].

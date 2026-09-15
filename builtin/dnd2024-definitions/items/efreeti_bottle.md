---
splat_schema: 1
splat_id: 2184a1f9-c762-552d-918f-af1845528517
splat_kind: item
splat_key: dnd2024:items/efreeti_bottle
splat_name: Efreeti Bottle
splat_origin:
  key: dnd2024:reference/magic_item/efreeti_bottle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/efreeti_bottle
  implementation: dnd2024:item_efreeti_bottle
  parameters:
    action: magic
    activation: remove_stopper
    summon:
      at: actor.turn_end
      creature:
        key: dnd2024:creatures/efreeti
      space:
        unoccupied: true
        range_ft: 30
    first_use_roll:
      die: 10
      persistent_result: true
      rows:
      - range:
        - 1
        - 1
        behavior: attack_user
        duration:
          amount: 5
          unit: rounds
        end:
        - disappear
        - bottle_nonmagical
      - range:
        - 2
        - 9
        understands: user_languages
        behavior: obey_commands
        duration:
          amount: 1
          unit: hours
        end:
        - return_to_bottle
        - new_stopper
        stopper_locked:
          amount: 24
          unit: hours
        repeat_on_openings:
        - 2
        - 3
        opening_4:
        - escape
        - disappear
        - bottle_nonmagical
      - range:
        - 10
        - 10
        understands: user_languages
        spell:
          spell:
            key: dnd2024:spells/wish
          uses: 1
          beneficiary: user
        ends_on_any:
        - wish_granted
        - one_hour_elapsed
        end:
        - disappear
        - bottle_nonmagical
    attunement_required: false
splat_contracts:
- id: dnd2024:item_efreeti_bottle
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
  - turn.ended
  - time.elapsed
  - spell.cast_completed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - effect_duration
  - spatial_targeting
  - summoning
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: cf1c2e8729b5fce36ae0d8a1adb57fe08158bbc6a42653d6d04d5d2692937b6b
  source_path: builtin/srd-5.2.1/magic_item/efreeti_bottle.md
  pages:
  - 220
  engine_gaps: &id001
  - dnd2024:item_efreeti_bottle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: d092512d7e1e159b0368334509ce3488996c43ff7b7a9db2fc77830b4828927d
splat_clauses:
- key: source_rules
  text: '# Efreeti Bottle Wondrous Item, Very Rare When you take a Magic action to remove the stopper
    of this painted brass bottle, a cloud of thick smoke flows out of it. At the end of your turn, the
    smoke disappears with a flash of harmless fire, and an Efreeti appears in an unoccupied space within
    30 feet of you. The first time the bottle is opened, the GM rolls on the following table to determine
    what happens. 1d10 Effect 1 The efreeti attacks you. After fighting for 5 rounds, the efreeti disappears,
    and the bottle loses its magic. 2–9 The efreeti understands your languages and obeys your commands
    for 1 hour, after which it returns to the bottle, and a new stopper contains it. The stopper can’t
    be removed for 24 hours. The next two times the bottle is opened, the same effect occurs. If the bottle
    is opened a fourth time, the efreeti escapes and disappears, and the bottle loses its magic. 10 The
    efreeti understands your languages and can cast Wish once for you. It disappears when it grants the
    wish or after 1 hour, and the bottle loses its magic.'
  sha256: 276ee645d6e2759dfa2f54445cccb034772d5196bf530fcdb5768161e6368dbb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Efreeti Bottle

<!-- SRD page 220 -->

Wondrous Item, Very Rare\
When you take a Magic action to remove the stopper\
of this painted brass bottle, a cloud of thick smoke\
flows out of it. At the end of your turn, the smoke dis-\
appears with a flash of harmless fire, and an Efreeti\
appears in an unoccupied space within 30 feet of you.\
The first time the bottle is opened, the GM rolls on\
the following table to determine what happens.\
1d10 Effect\
1 The efreeti attacks you. After fighting for 5\
rounds, the efreeti disappears, and the bottle\
loses its magic.\
2–9 The efreeti understands your languages and\
obeys your commands for 1 hour, after which\
it returns to the bottle, and a new stopper\
contains it. The stopper can’t be removed for\
24 hours. The next two times the bottle is\
opened, the same effect occurs. If the bottle is\
opened a fourth time, the efreeti escapes and\
disappears, and the bottle loses its magic.\
10 The efreeti understands your languages and can\
cast Wish once for you. It disappears when it\
grants the wish or after 1 hour, and the bottle\
loses its magic.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].

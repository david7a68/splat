---
splat_schema: 1
splat_id: 689f867d-12da-56e3-9b94-78f91528f9bb
splat_kind: item
splat_key: dnd2024:items/gem_of_brightness
splat_name: Gem of Brightness
splat_origin:
  key: dnd2024:reference/magic_item/gem_of_brightness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/gem_of_brightness
  implementation: dnd2024:item_gem_of_brightness
  parameters:
    equipped: held
    charges:
      initial: 50
      maximum: 50
      recovery: null
    action: magic
    command_words:
    - word: first
      charge_cost: 0
      light:
        bright_radius_ft: 30
        additional_dim_ft: 30
      ends_on:
      - bonus_action_repeat_first_word
      - another_gem_function
    - word: second
      charge_cost: 1
      target:
        visible: true
        range_ft: 60
        creature_count: 1
      save:
        ability: constitution
        dc: 15
        failure:
          condition: blinded
          duration:
            amount: 1
            unit: minutes
        repeat:
          event: target.turn_end
          success: end_effect
    - word: third
      charge_cost: 5
      area:
        shape: cone
        size_ft: 30
      save:
        ability: constitution
        dc: 15
        failure:
          condition: blinded
          duration:
            amount: 1
            unit: minutes
        repeat:
          event: target.turn_end
          success: end_effect
    at_zero_charges:
      item_magic: false
      jewel_value_gp: 50
    attunement_required: false
splat_contracts:
- id: dnd2024:item_gem_of_brightness
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
  capabilities:
  - item_effects
  - item_charges
  - effect_duration
  - spatial_targeting
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9e31cf277ae018c32784ce7fb9c675b8ba463a1dafa3bd9a6c54fccabceb5f74
  source_path: builtin/srd-5.2.1/magic_item/gem_of_brightness.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_gem_of_brightness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: c62114bf2a4c91f046a50d8d6e8545428d4cbb76ae1cce16c3df3fc69637e9f4
splat_clauses:
- key: source_rules
  text: '# Gem of Brightness Wondrous Item, Uncommon This prism has 50 charges. While you are holding
    it, you can take a Magic action and use one of three command words to cause one of the following effects:
    First Command Word. The gem sheds Bright Light in a 30-foot radius and Dim Light for an additional
    30 feet. This effect doesn’t expend a charge. It lasts until you take a Bonus Action to repeat the
    command word or until you use another function of the gem. Second Command Word. You expend 1 charge
    and cause the gem to fire a brilliant beam of light at one creature you can see within 60 feet of
    yourself. The creature must succeed on a DC 15 Constitution saving throw or have the Blinded condition
    for 1 minute. The creature repeats the save at the end of each of its turns, ending the effect on
    itself on a success. Third Command Word. You expend 5 charges and cause the gem to flare with intense
    light in a 30- foot Cone. Each creature in the Cone makes a saving throw as if struck by the beam
    created with the second command word. When all of the gem’s charges are expended, the gem becomes
    a nonmagical jewel worth 50 GP.'
  sha256: 9df57a3aba423b6e4315048a62956d7928352ad9fedca09e11f3917db64129b7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gem of Brightness

<!-- SRD page 223 -->

Wondrous Item, Uncommon\
This prism has 50 charges. While you are holding\
it, you can take a Magic action and use one of three\
command words to cause one of the following\
effects:\
First Command Word. The gem sheds Bright Light\
in a 30-foot radius and Dim Light for an additional\
30 feet. This effect doesn’t expend a charge. It\
lasts until you take a Bonus Action to repeat the\
command word or until you use another function\
of the gem.\
Second Command Word. You expend 1 charge and\
cause the gem to fire a brilliant beam of light at\
one creature you can see within 60 feet of your-\
self. The creature must succeed on a DC 15 Consti-\
tution saving throw or have the Blinded condition\
for 1 minute. The creature repeats the save at the\
end of each of its turns, ending the effect on itself\
on a success.\
Third Command Word. You expend 5 charges and\
cause the gem to flare with intense light in a 30-\
foot Cone. Each creature in the Cone makes a sav-\
ing throw as if struck by the beam created with\
the second command word.\
When all of the gem’s charges are expended, the\
gem becomes a nonmagical jewel worth 50 GP.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

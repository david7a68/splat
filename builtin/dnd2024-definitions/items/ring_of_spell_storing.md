---
splat_schema: 2
splat_id: d6cc876e-c146-5ef9-a1f6-ad5d4ec5e569
splat_kind: item
splat_key: dnd2024:items/ring_of_spell_storing
splat_name: Ring of Spell Storing
splat_origin: dnd2024:reference/magic_item/ring_of_spell_storing
splat_sources:
- SRD 5.2.1, pp. 239, https://www.dndbeyond.com/srd
---

# Ring of Spell Storing


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 239 -->

Ring, Rare (Requires Attunement)

This ring stores spells cast into it, holding them until the attuned wearer uses them. The ring can store up to 5 levels worth of spells at a time. When found, it contains 1d6 − 1 levels of stored spells chosen by the GM.

Any creature can cast a spell of level 1 through 5 into the ring by touching the ring as the spell is cast. The spell has no effect other than to be stored in the ring. If the ring can’t hold the spell, the spell is expended without effect. The level of the slot used to cast the spell determines how much space it uses.

While wearing this ring, you can cast any spell stored in it. The spell uses the slot level, spell save DC, spell attack bonus, and spellcasting ability of the original caster but is otherwise treated as if you cast the spell. The spell cast from the ring is no longer stored in it, freeing up space.

Original layout: [[../../srd-5.2.1/pages/239|page 239]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_spell_storing ` |
| ` implementation ` | ` dnd2024:item_ring_of_spell_storing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Spell storage

<!-- splat:yaml path=splat_rule/parameters/spell_storage -->

```yaml
capacity_spell_levels: 5
initial_spell_levels:
  roll:
    count: 1
    sides: 6
    add: -1
initial_spells_choice: gm
store:
  caster: any_creature
  touch_required_during_cast: true
  spell_level_min: 1
  spell_level_max: 5
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
```

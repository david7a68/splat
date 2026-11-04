---
splat_schema: 2
splat_id: fc71ebbf-7389-5ddc-9c61-c6ca3fce7978
splat_kind: spell
splat_key: dnd2024:spells/compulsion
splat_name: Compulsion
splat_origin: dnd2024:reference/spell/compulsion
splat_sources:
- SRD 5.2.1, pp. 117, https://www.dndbeyond.com/srd
---

# Compulsion


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Enchantment |
| Classes | Bard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 117 -->

Level 4 Enchantment (Bard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

Each creature of your choice that you can see within range must succeed on a Wisdom saving throw or have the Charmed condition until the spell ends.

For the duration, you can take a Bonus Action to designate a direction that is horizontal to you. Each Charmed target must use as much of its movement as possible to move in that direction on its next turn, taking the safest route. After moving in this way, a target repeats the save, ending the spell on itself on a success.

Original layout: [[../../srd-5.2.1/pages/117|page 117]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/compulsion ` |
| ` implementation ` | ` dnd2024:spell_compulsion ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` chosen_all ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
success:
  effect: none
```

#### Command

<!-- splat:yaml path=splat_rule/parameters/command -->

```yaml
action: bonus_action
direction: caster_chosen_horizontal
targets: charmed_targets
timing: next_target_turn
movement: as_much_as_possible
route: safest
after_movement_save:
  ability: wisdom
  success: end_spell_on_self
```

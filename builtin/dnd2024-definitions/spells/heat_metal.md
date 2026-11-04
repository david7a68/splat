---
splat_schema: 2
splat_id: 7ecefdcd-9575-5bde-a625-51c97bc691db
splat_kind: spell
splat_key: dnd2024:spells/heat_metal
splat_name: Heat Metal
splat_origin: dnd2024:reference/spell/heat_metal
splat_sources:
- SRD 5.2.1, pp. 139, 140, https://www.dndbeyond.com/srd
---

# Heat Metal


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Bard, Druid |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a piece of iron and a flame) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 139 -->

Level 2 Transmutation (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a piece of iron and a flame)\
Duration: Concentration, up to 1 minute

<!-- SRD page 140 -->

Choose a manufactured metal object, such as a metal weapon or a suit of Heavy or Medium metal armor, that you can see within range. You cause the object to glow red-hot. Any creature in physical contact with the object takes 2d8 Fire damage when you cast the spell. Until the spell ends, you can take a Bonus Action on each of your later turns to deal this damage again if the object is within range.

If a creature is holding or wearing the object and takes the damage from it, the creature must succeed on a Constitution saving throw or drop the object if it can. If it doesn’t drop the object, it has Disadvantage on attack rolls and ability checks until the start of your next turn.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/139|page 139]], [[../../srd-5.2.1/pages/140|page 140]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/heat_metal ` |
| ` implementation ` | ` dnd2024:spell_heat_metal ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` manufactured ` | ` true ` |
| ` material ` | ` metal ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Object state

<!-- splat:values path=splat_rule/parameters/object_state -->

| Field | Value |
| --- | --- |
| ` red_hot_glow ` | ` true ` |
| ` duration ` | ` spell_duration ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
targets: creatures_physically_contacting_object
dice: 2d8
type: fire
initial: on_cast
repeat:
  economy: bonus_action
  timing: each_later_caster_turn
  requires: object_within_range
```

#### Holder or wearer damaged

<!-- splat:yaml path=splat_rule/parameters/holder_or_wearer_damaged -->

```yaml
save:
  ability: constitution
  failure: drop_object_if_able
if_not_dropped:
  disadvantage:
  - attack_rolls
  - ability_checks
  until: caster_next_turn_start
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```


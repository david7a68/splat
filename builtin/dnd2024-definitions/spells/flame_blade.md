---
splat_schema: 2
splat_id: 3c2661a3-d5fe-56e7-9c6e-8e5439258de8
splat_kind: spell
splat_key: dnd2024:spells/flame_blade
splat_name: Flame Blade
splat_origin: dnd2024:reference/spell/flame_blade
splat_sources:
- SRD 5.2.1, pp. 132, https://www.dndbeyond.com/srd
---

# Flame Blade


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Evocation |
| Classes | Druid, Sorcerer |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S, M (a sumac leaf) |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 132 -->

Level 2 Evocation (Druid, Sorcerer)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S, M (a sumac leaf)\
Duration: Concentration, up to 10 minutes

You evoke a fiery blade in your free hand. The blade is similar in size and shape to a scimitar, and it lasts for the duration. If you let go of the blade, it disappears, but you can evoke it again as a Bonus Action.

As a Magic action, you can make a melee spell attack with the fiery blade. On a hit, the target takes Fire damage equal to 3d6 plus your spellcasting ability modifier.

The flaming blade sheds Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/132|page 132]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/flame_blade ` |
| ` implementation ` | ` dnd2024:spell_flame_blade ` |

### Parameters

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
requires_free_hand: true
form: scimitar_sized_shaped_blade
duration: spell_duration
release: disappear
reevoke_cost: bonus_action
light:
  bright_radius_feet: 10
  additional_dim_feet: 10
```

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
cost: magic_action
kind: melee_spell
hit:
  dice:
    count:
      op: add
      args:
      - 3
      - op: multiply
        args:
        - 1
        - op: subtract
          args:
          - binding: cast_level
          - 2
    sides: 6
    add:
      binding: spellcasting_modifier
  type: fire
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```


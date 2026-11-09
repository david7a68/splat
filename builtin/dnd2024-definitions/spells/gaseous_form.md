---
splat_schema: 2
splat_id: cfc106b6-22ae-589d-a591-314d0166fc92
splat_kind: spell
splat_key: dnd2024:spells/gaseous_form
splat_name: Gaseous Form
splat_origin: dnd2024:reference/spell/gaseous_form
splat_sources:
- SRD 5.2.1, pp. 134, 135, https://www.dndbeyond.com/srd
---

# Gaseous Form


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Transmutation |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a bit of gauze) |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 134 -->

Level 3 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a bit of gauze)\
Duration: Concentration, up to 1 hour

A willing creature you touch shape-shifts, along with everything it’s wearing and carrying, into a misty cloud for the duration. The spell ends on the target if it drops to 0 Hit Points or if it takes a Magic action to end the spell on itself.

<!-- SRD page 135 -->

While in this form, the target’s only method of movement is a Fly Speed of 10 feet, and it can hover. The target can enter and occupy the space of another creature. The target has Resistance to Bludgeoning, Piercing, and Slashing damage; it has Immunity to the Prone condition; and it has Advantage on Strength, Dexterity, and Constitution saving throws. The target can pass through narrow openings, but it treats liquids as though they were solid surfaces.

The target can’t talk or manipulate objects, and any objects it was carrying or holding can’t be dropped, used, or otherwise interacted with. Finally, the target can’t attack or cast spells.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/134|page 134]], [[../../srd-5.2.1/pages/135|page 135]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/gaseous_form ` |
| ` implementation ` | ` dnd2024:spell_gaseous_form ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
maximum:
  op: add
  args:
  - 1
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 3
within_spell_range: true
willing: true
touched: true
```

#### Transformation

<!-- splat:yaml path=splat_rule/parameters/transformation -->

```yaml
form: misty_cloud
include_worn_carried: true
duration: spell_duration
only_movement:
  fly_speed_feet: 10
  hover: true
may_enter_and_occupy_other_creature_space: true
damage_resistance:
- bludgeoning
- piercing
- slashing
condition_immunity:
- prone
save_advantage:
- strength
- dexterity
- constitution
pass_narrow_openings: true
liquids_treated_as: solid_surfaces
prohibit:
- talk
- manipulate_objects
- drop_carried_or_held_objects
- use_carried_or_held_objects
- interact_with_carried_or_held_objects
- attack
- cast_spells
```

#### End on target

<!-- splat:values path=splat_rule/parameters/end_on_target -->

| Field | Value |
| --- | --- |
| ` hp_zero ` | ` true ` |
| ` voluntary_cost ` | ` magic_action ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```


---
splat_schema: 2
splat_id: 78179ce1-92b8-56eb-a470-54ca77355e39
splat_kind: spell
splat_key: dnd2024:spells/dancing_lights
splat_name: Dancing Lights
splat_origin: dnd2024:reference/spell/dancing_lights
splat_sources:
- SRD 5.2.1, pp. 121, 122, https://www.dndbeyond.com/srd
---

# Dancing Lights


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Illusion |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a bit of phosphorus) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 121 -->

Illusion Cantrip (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a bit of phosphorus)\
Duration: Concentration, up to 1 minute

<!-- SRD page 122 -->

You create up to four torch-size lights within range, making them appear as torches, lanterns, or glowing orbs that hover for the duration. Alternatively, you combine the four lights into one glowing Medium form that is vaguely humanlike. Whichever form you choose, each light sheds Dim Light in a 10-foot radius.

As a Bonus Action, you can move the lights up to 60 feet to a space within range. A light must be within 20 feet of another light created by this spell, and a light vanishes if it exceeds the spell’s range.

Original layout: [[../../srd-5.2.1/pages/121|page 121]], [[../../srd-5.2.1/pages/122|page 122]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dancing_lights ` |
| ` implementation ` | ` dnd2024:spell_dancing_lights ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` locations ` |
| ` within_range ` | ` true ` |

#### Lights

<!-- splat:yaml path=splat_rule/parameters/lights -->

```yaml
max_count: 4
size: torch
appearance_choice:
- torch
- lantern
- glowing_orb
hover: true
alternative_combined_form:
  count: 1
  size: medium
  shape: vaguely_humanlike
illumination_each:
  kind: dim
  radius_feet: 10
duration: spell_duration
```

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` max_feet ` | ` 60 ` |
| ` destination_within_range ` | ` true ` |

#### Constraints

<!-- splat:values path=splat_rule/parameters/constraints -->

| Field | Value |
| --- | --- |
| ` max_distance_to_another_light_feet ` | ` 20 ` |
| ` beyond_spell_range ` | ` light_vanishes ` |

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


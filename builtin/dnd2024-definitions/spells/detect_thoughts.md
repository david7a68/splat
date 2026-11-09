---
splat_schema: 2
splat_id: 38bc921e-b3ab-5d83-a3d6-c2bccda83ee3
splat_kind: spell
splat_key: dnd2024:spells/detect_thoughts
splat_name: Detect Thoughts
splat_origin: dnd2024:reference/spell/detect_thoughts
splat_sources:
- SRD 5.2.1, pp. 123, https://www.dndbeyond.com/srd
---

# Detect Thoughts


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Divination |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (1 Copper Piece) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 123 -->

Level 2 Divination (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (1 Copper Piece)\
Duration: Concentration, up to 1 minute

You activate one of the effects below. Until the spell ends, you can activate either effect as a Magic action on your later turns.

Sense Thoughts. You sense the presence of thoughts within 30 feet of yourself that belong to creatures that know languages or are telepathic. You don’t read the thoughts, but you know that a thinking creature is present.

The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal; or a thin sheet of lead.

Read Thoughts. Target one creature you can see within 30 feet of yourself or one creature within 30 feet of yourself that you detected with the Sense Thoughts option. You learn what is most on the target’s mind right now. If the target doesn’t know any languages and isn’t telepathic, you learn nothing.

As a Magic action on your next turn, you can try to probe deeper into the target’s mind. If you probe deeper, the target makes a Wisdom saving throw. On a failed save, you discern the target’s reasoning, emotions, and something that looms large in its mind (such as a worry, love, or hate). On a successful save, the spell ends. Either way, the target knows that you are probing into its mind, and until you shift your attention away from the target’s mind, the target can take an action on its turn to make an Intelligence (Arcana) check against your spell save DC, ending the spell on a success.

Original layout: [[../../srd-5.2.1/pages/123|page 123]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/detect_thoughts ` |
| ` implementation ` | ` dnd2024:spell_detect_thoughts ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` barrier_scope ` | ` spell ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
at_cast: choose_one_option
later:
  economy: magic_action
  timing: caster_later_turn
  options:
  - sense_thoughts
  - read_thoughts
```

#### Sense thoughts

<!-- splat:yaml path=splat_rule/parameters/sense_thoughts -->

```yaml
range_feet: 30
eligible:
  any:
  - knows_language
  - telepathic
output: thinking_creature_presence
read_content: false
```

#### Read thoughts

<!-- splat:yaml path=splat_rule/parameters/read_thoughts -->

```yaml
targets: 1
range_feet: 30
eligible:
  any:
  - visible
  - detected_with_sense_thoughts
output: foremost_current_thought
no_language_and_not_telepathic_output: nothing
```

#### Probe

<!-- splat:yaml path=splat_rule/parameters/probe -->

```yaml
economy: magic_action
timing: next_caster_turn_after_read
target: read_target
save:
  ability: wisdom
  dc: spell_save_dc
failure_output:
- reasoning
- emotions
- prominent_concern
success: end_spell
either_outcome: target_knows_probe
```

#### Resistance after probe

<!-- splat:yaml path=splat_rule/parameters/resistance_after_probe -->

```yaml
until: attention_shifts_to_other_target
economy: action
timing: target_turn
check:
  ability: intelligence
  skill: arcana
  dc: spell_save_dc
success: end_spell
```

#### Barriers

<!-- splat:table
path: splat_rule/parameters/barriers
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Material:
    path: material
    type: symbol
  Thickness inches:
    path: thickness_inches
    type: integer
    optional: true
  Thickness:
    path: thickness
    type: symbol
    optional: true
-->

| Key | Material | Thickness inches | Thickness |
| --- | --- | --- | --- |
| ` row_1 ` | ` Stone ` | ` 12 ` |  |
| ` row_2 ` | ` Dirt ` | ` 12 ` |  |
| ` row_3 ` | ` Wood ` | ` 12 ` |  |
| ` row_4 ` | ` Metal ` | ` 1 ` |  |
| ` row_5 ` | ` Lead ` |  | ` Any sheet ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```


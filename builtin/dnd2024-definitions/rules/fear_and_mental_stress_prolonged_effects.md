---
splat_schema: 1
splat_id: ae96b156-df76-5e73-9bcb-fef056e1f8fe
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_prolonged_effects
splat_name: Prolonged Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
---

# Prolonged Effects

## Source text

<!-- SRD page 196 -->

Exposure to mental stress can cause prolonged effects. Consider the following possibilities.

Short-Term Effects. The character has the Frightened, Incapacitated, or Stunned condition for 1d10 minutes. This condition might be accompanied by alarming behavior or hallucinations. These effects can be suppressed with the Calm Emotions spell or removed by the Lesser Restoration spell.

Long-Term Effects. The character has Disadvantage on some or all ability checks for 1d10 × 10 hours, stemming from an unwillingness or inability to exert a particular set of abilities. The character might feel enervated and unable to exert much Strength, for example, or become so suspicious of others that Charisma checks are more difficult. These effects can be suppressed with the Calm Emotions spell or removed by the Lesser Restoration spell.

Indefinite Effects. An indefinite effect is a long-term effect (see above) that lasts until removed by a Greater Restoration spell. It can be suppressed by a Calm Emotions spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/fear_and_mental_stress_prolonged_effects ` |
| ` implementation ` | ` dnd2024:rules_fear_and_mental_stress_prolonged_effects ` |

### Parameters

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` cause ` | ` mental_stress ` |
| ` choice ` | ` ["short_term", "long_term", "indefinite"] ` |

#### Short term

<!-- splat:yaml path=splat_rule/parameters/short_term -->

```yaml
condition_choice:
- frightened
- incapacitated
- stunned
duration:
  dice:
    count: 1
    sides: 10
  unit: minute
optional_narrative:
- alarming_behavior
- hallucinations
suppressed_by:
  key: dnd2024:spells/calm_emotions
removed_by:
  key: dnd2024:spells/lesser_restoration
```

#### Long term

<!-- splat:yaml path=splat_rule/parameters/long_term -->

```yaml
disadvantage: selected_ability_checks
ability_scope:
  authority: gm
  choose_min: 1
  choose_max: 6
  options:
  - strength
  - dexterity
  - constitution
  - intelligence
  - wisdom
  - charisma
duration:
  dice:
    count: 1
    sides: 10
  multiplier: 10
  unit: hour
suppressed_by:
  key: dnd2024:spells/calm_emotions
removed_by:
  key: dnd2024:spells/lesser_restoration
```

#### Indefinite

<!-- splat:yaml path=splat_rule/parameters/indefinite -->

```yaml
effect_template: long_term
duration: until_removed
suppressed_by:
  key: dnd2024:spells/calm_emotions
removed_by:
  key: dnd2024:spells/greater_restoration
lesser_restoration_removes: false
```

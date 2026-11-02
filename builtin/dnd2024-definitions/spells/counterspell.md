---
splat_schema: 1
splat_id: 028154e6-b9bb-57af-b2a1-5990f575e950
splat_kind: spell
splat_key: dnd2024:spells/counterspell
splat_name: Counterspell
splat_origin:
  key: dnd2024:reference/spell/counterspell
splat_sources:
- title: SRD 5.2.1
  locator: pp. 120
  url: https://www.dndbeyond.com/srd
---

# Counterspell

## Source text

<!-- SRD page 120 -->

Level 3 Abjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Reaction, which you take when you see a creature within 60 feet of yourself casting a spell with Verbal, Somatic, or Material components\
Range: 60 feet\
Components: S\
Duration: Instantaneous

You attempt to interrupt a creature in the process of casting a spell. The creature makes a Constitution saving throw. On a failed save, the spell dissipates with no effect, and the action, Bonus Action, or Reaction used to cast it is wasted. If that spell was cast with a spell slot, the slot isn’t expended.

Original layout: [[../../srd-5.2.1/pages/120|page 120]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Reaction, which you take when you see a creature within 60 feet of yourself casting a spell with Verbal, Somatic, or Material components ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/counterspell ` |
| ` implementation ` | ` dnd2024:spell_counterspell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target ` | ` triggering_caster ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` spell.cast_pending ` |
| ` creature_visible ` | ` true ` |
| ` distance_feet_max ` | ` 60 ` |
| ` component_any ` | ` ["verbal", "somatic", "material"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` casting_continues ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
spell_effect: dissipate_without_effect
casting_economy: wasted
applicable_economy:
- action
- bonus_action
- reaction
slot_cast:
  spell_slot: not_expended
```

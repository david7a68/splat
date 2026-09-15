---
splat_schema: 1
splat_id: c193e0c7-d054-5703-8445-d8aec30d9827
splat_kind: spell
splat_key: dnd2024:spells/true_resurrection
splat_name: True Resurrection
splat_origin:
  key: dnd2024:reference/spell/true_resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 170, 171
  url: https://www.dndbeyond.com/srd
---

# True Resurrection

## Source text

<!-- SRD page 170 -->

Level 9 Necromancy (Cleric, Druid)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (diamonds worth 25,000+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You touch a creature that has been dead for no\
longer than 200 years and that died for any reason\
except old age. The creature is revived with all its\
Hit Points.\
This spell closes all wounds, neutralizes any\
poison, cures all magical contagions, and lifts any\
curses affecting the creature when it died. The spell\
replaces damaged or missing organs and limbs.\
If the creature was Undead, it is restored to its\
non-Undead form.\
The spell can provide a new body if the original\
no longer exists, in which case you must speak the\

<!-- SRD page 171 -->

creature’s name. The creature then appears in an\
unoccupied space you choose within 10 feet of you.\

Original layout: [[../../srd-5.2.1/pages/170|page 170]], [[../../srd-5.2.1/pages/171|page 171]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 hour ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` diamonds worth 25,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/true_resurrection ` |
| ` implementation ` | ` dnd2024:spell_true_resurrection ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` dead_maximum_years ` | ` 200 ` |
| ` death_by_old_age ` | ` false ` |
| ` touch_required_if_body_exists ` | ` true ` |

#### Revive

<!-- splat:values path=splat_rule/parameters/revive -->

| Field | Value |
| --- | --- |
| ` hp ` | ` full ` |
| ` close_all_wounds ` | ` true ` |
| ` neutralize_poison ` | ` true ` |
| ` cure_all_magical_contagions ` | ` true ` |
| ` lift_curses_at_death ` | ` true ` |
| ` replace_damaged_missing ` | ` ["organs", "limbs"] ` |
| ` if_undead ` | ` restore_nonundead_form ` |

#### Missing body

<!-- splat:values path=splat_rule/parameters/missing_body -->

| Field | Value |
| --- | --- |
| ` create_new_body ` | ` true ` |
| ` speak_name_required ` | ` true ` |
| ` appearance_space ` | ` caster_chosen_unoccupied ` |
| ` within_caster_feet ` | ` 10 ` |

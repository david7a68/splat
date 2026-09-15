---
splat_schema: 1
splat_id: edf1ef04-81a2-55ba-98f0-e742a7834c1f
splat_kind: spell
splat_key: dnd2024:spells/resurrection
splat_name: Resurrection
splat_origin:
  key: dnd2024:reference/spell/resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158, 159
  url: https://www.dndbeyond.com/srd
---

# Resurrection

## Source text

<!-- SRD page 158 -->

Level 7 Necromancy (Bard, Cleric)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP,\
which the spell consumes)\
Duration: Instantaneous\

<!-- SRD page 159 -->

With a touch, you revive a dead creature that has\
been dead for no more than a century, didn’t die of\
old age, and wasn’t Undead when it died.\
The creature returns to life with all its Hit Points.\
This spell also neutralizes any poisons that affected\
the creature at the time of death. This spell closes\
all mortal wounds and restores any missing body\
parts.\
Coming back from the dead is an ordeal. The tar-\
get takes a −4 penalty to D20 Tests. Every time the\
target finishes a Long Rest, the penalty is reduced\
by 1 until it becomes 0.\
Casting this spell to revive a creature that has\
been dead for 365 days or longer taxes you. Until\
you finish a Long Rest, you can’t cast spells again,\
and you have Disadvantage on D20 Tests.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]], [[../../srd-5.2.1/pages/159|page 159]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["bard", "cleric"] ` |
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
| ` description ` | ` a diamond worth 1,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/resurrection ` |
| ` implementation ` | ` dnd2024:spell_resurrection ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` dead_creature ` |
| ` touched ` | ` true ` |
| ` dead_years_maximum ` | ` 100 ` |
| ` death_from_old_age ` | ` false ` |
| ` undead_when_died ` | ` false ` |

#### Revive

<!-- splat:values path=splat_rule/parameters/revive -->

| Field | Value |
| --- | --- |
| ` hp ` | ` all ` |
| ` neutralize ` | ` poisons_at_time_of_death ` |
| ` close ` | ` all_mortal_wounds ` |
| ` restore ` | ` missing_body_parts ` |

#### Target penalty

<!-- splat:yaml path=splat_rule/parameters/target_penalty -->

```yaml
rolls: d20_tests
initial: -4
recovery:
  event: target_long_rest_completed
  increase: 1
  maximum: 0
```

#### Caster costly revival

<!-- splat:values path=splat_rule/parameters/caster_costly_revival -->

| Field | Value |
| --- | --- |
| ` requires_dead_days_minimum ` | ` 365 ` |
| ` prohibit ` | ` cast_spells ` |
| ` disadvantage ` | ` d20_tests ` |
| ` until ` | ` caster_long_rest_completed ` |

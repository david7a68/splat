---
splat_schema: 1
splat_id: 9e18cd93-af64-5457-baff-8b6bded8b9b7
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_sewer_plague
splat_name: Sewer Plague
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
---

# Sewer Plague

## Source text

<!-- SRD page 194 -->

Magical Contagion\
Fouled potions and alchemical waste can give rise\
to Sewer Plague, which incubates in sewers and\
refuse heaps and is sometimes transmitted by\
creatures that dwell in such areas, including otyu-\
ghs and rats. Any Humanoid that is wounded by a\
creature that carries the contagion or that comes\
into contact with contaminated filth or offal must\
succeed on a DC 11 Constitution saving throw or be-\
come infected with Sewer Plague. A creature suffers\
the following effects 1d4 days after infection:\
Fatigue. The creature gains 1 Exhaustion level.\
Weakness. While the creature has any Exhaustion\
levels, it regains only half the normal number of\
Hit Points from spending Hit Point Dice.\
Restlessness. While the creature has any Exhaus-\
tion levels, finishing a Long Rest neither restores\
lost Hit Points nor reduces the creature’s Exhaus-\
tion level.\
Fighting the Contagion. Daily at dawn, an in-\
fected creature makes a DC 11 Constitution saving\
throw. On a failed save, the creature gains 1 Ex-\
haustion level as its fatigue worsens. On a success-\
ful save, the creature’s Exhaustion level decreases\
by 1. If the creature’s Exhaustion level is reduced to\
0, the contagion ends on the creature.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_sewer_plague ` |
| ` implementation ` | ` dnd2024:rules_magical_contagions_sewer_plague ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` kind ` | ` magical_contagion ` |

#### Infection

<!-- splat:yaml path=splat_rule/parameters/infection -->

```yaml
target_type: humanoid
exposure:
- wounded_by_infected_carrier
- contact_contaminated_filth_or_offal
save:
  ability: constitution
  dc: 11
on_failure: infected
onset:
  dice:
    count: 1
    sides: 4
  unit: day
```

#### Onset effect

<!-- splat:values path=splat_rule/parameters/onset_effect -->

| Field | Value |
| --- | --- |
| ` exhaustion_gain ` | ` 1 ` |

#### While exhaustion positive

<!-- splat:values path=splat_rule/parameters/while_exhaustion_positive -->

| Field | Value |
| --- | --- |
| ` hit_die_healing_multiplier ` | ` 0.5 ` |
| ` long_rest_restores_hp ` | ` false ` |
| ` long_rest_reduces_exhaustion ` | ` false ` |

#### Recovery

<!-- splat:yaml path=splat_rule/parameters/recovery -->

```yaml
timing: daily_dawn
save:
  ability: constitution
  dc: 11
failure:
  exhaustion_gain: 1
success:
  exhaustion_reduce: 1
  end_contagion_if_exhaustion_zero: true
```

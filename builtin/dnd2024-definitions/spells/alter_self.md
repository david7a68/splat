---
splat_schema: 2
splat_id: e12db40f-cc6c-5bad-8299-f6c5898caf0e
splat_kind: spell
splat_key: dnd2024:spells/alter_self
splat_name: Alter Self
splat_origin: dnd2024:reference/spell/alter_self
splat_sources:
- SRD 5.2.1, pp. 107, https://www.dndbeyond.com/srd
---

# Alter Self


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 107 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 hour

You alter your physical form. Choose one of the following options. Its effects last for the duration, during which you can take a Magic action to replace the option you chose with a different one.

Aquatic Adaptation. You sprout gills and grow webs between your fingers. You can breathe underwater and gain a Swim Speed equal to your Speed.

Change Appearance. You alter your appearance. You decide what you look like, including your height, weight, facial features, sound of your voice, hair length, coloration, and other distinguishing characteristics. You can make yourself appear as a member of another species, though none of your statistics change. You can’t appear as a creature of a different size, and your basic shape stays the same; if you’re bipedal, you can’t use this spell to become quadrupedal, for instance. For the duration, you can take a Magic action to change your appearance in this way again.

Natural Weapons. You grow claws (Slashing), fangs (Piercing), horns (Piercing), or hooves (Bludgeoning). When you use your Unarmed Strike to deal damage with that new growth, it deals 1d6 damage of the type in parentheses instead of dealing the normal damage for your Unarmed Strike, and you use your spellcasting ability modifier for the attack and damage rolls rather than using Strength.

Original layout: [[../../srd-5.2.1/pages/107|page 107]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/alter_self ` |
| ` implementation ` | ` dnd2024:spell_alter_self ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` count ` | ` 1 ` |
| ` options ` | ` ["aquatic_adaptation", "change_appearance", "natural_weapons"] ` |
| ` replace_action ` | ` magic ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
aquatic_adaptation:
  underwater_breathing: true
  swim_speed:
    stat:
      of: caster
      key: speed
change_appearance:
  choice_fields:
  - height
  - weight
  - face
  - voice
  - hair_length
  - color
  - distinguishing_features
  - apparent_species
  statistics_change: false
  same_size_category: true
  same_basic_shape: true
  change_action: magic
natural_weapons:
  growth_choice:
    claws: slashing
    fangs: piercing
    horns: piercing
    hooves: bludgeoning
  attack: unarmed_strike
  damage_replaces_normal:
    dice:
      count: 1
      sides: 6
      add:
        binding: spellcasting_modifier
  attack_ability: spellcasting
  damage_ability: spellcasting
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```


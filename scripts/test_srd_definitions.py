"""Source and failure-boundary tests for the full catalogue pass."""
import collections
import json
import unittest
from pathlib import Path
import yaml
from convert_srd import load_note
from convert_srd_definitions import (SOURCE, OUTPUT, CONTRACTS, spell_fields,
                                    creature_fields, equipment_tables, progression_tables)


class DefinitionTests(unittest.TestCase):
    def test_every_spell_header_is_converted_from_official_source(self):
        spells=list((SOURCE/'spell').glob('*.md'))
        self.assertEqual(len(spells),339)
        for path in spells:
            with self.subTest(path=path):
                _,body=load_note(path)
                spell=spell_fields(body)['splat_spell']
                self.assertIn(spell['level'],range(10))
                self.assertTrue(spell['components'])
        _,body=load_note(SOURCE/'spell/searing_smite.md')
        spell=spell_fields(body)['splat_spell']
        self.assertFalse(spell['concentration'])
        self.assertIn('immediately after hitting',spell['casting_time'])
        self.assertEqual(spell['duration'],'1 minute')

    def test_variable_hit_points_and_ambiguous_save_are_not_guessed(self):
        _,body=load_note(SOURCE/'creature/animated_object.md')
        fields=creature_fields(body)
        self.assertNotIn('max_hp',fields['splat_stats'])
        _,body=load_note(SOURCE/'creature/young_white_dragon.md')
        fields=creature_fields(body)
        self.assertEqual(fields['splat_stats']['max_hp']['value'],123)
        self.assertNotIn('save_intelligence',fields['splat_stats'])
        self.assertEqual(fields['splat_creature']['ambiguous_saves'],['intelligence'])

    def test_weapon_and_armor_tables_preserve_special_cases(self):
        rows={r['name']:r['fields']['splat_item'] for r in equipment_tables()}
        self.assertEqual(len(rows),51)
        self.assertEqual(rows['Dart']['mass'],{'value':0.25,'unit':'lb'})
        self.assertIn('unless mounted',rows['Lance']['properties_text'])
        self.assertIn('Two-Handed',rows['Heavy Crossbow']['properties_text'])
        self.assertEqual(rows['Plate Armor']['armor']['base_ac'],18)
        self.assertEqual(rows['Plate Armor']['armor']['strength_minimum'],15)
        self.assertEqual(rows['Shield']['armor']['ac_bonus'],2)
        self.assertEqual(rows['Half Plate Armor']['armor']['dexterity_modifier_max'],2)

    def test_all_class_levels_and_distinct_slot_models(self):
        tables={r['name']:r['fields']['splat_rule']['parameters']['rows'] for r in progression_tables()}
        self.assertEqual(len(tables),12)
        self.assertTrue(all(len(t)==20 for t in tables.values()))
        self.assertEqual(tables['Wizard Progression'][19]['spell_slots_9'],1)
        self.assertEqual(tables['Warlock Progression'][19]['pact_slots'],4)
        self.assertEqual(tables['Warlock Progression'][19]['pact_slot_level'],5)
        self.assertEqual(tables['Barbarian Progression'][0]['rages'],2)
        self.assertEqual(tables['Barbarian Progression'][19]['rage_damage'],4)
        self.assertEqual(tables['Monk Progression'][1]['unarmored_movement'],10)
        self.assertEqual(tables['Rogue Progression'][19]['sneak_attack'],'10d6')

    def test_revised_complex_contracts_preserve_rule_outcomes(self):
        specs=yaml.safe_load(CONTRACTS.read_text())
        counter=specs['dnd2024:spells/counterspell']['fields']['splat_actions']['cast']['steps'][0]['parameters']
        self.assertEqual(counter['save_ability'],'constitution')
        self.assertEqual(counter['failure']['spell_slot'],'not_expended')
        poly=specs['dnd2024:spells/polymorph']['fields']['splat_actions']['cast']['steps'][0]['parameters']
        self.assertIn('hit_points',poly['retained'])
        self.assertEqual(poly['temporary_hp'],'form_hit_points')
        hydra=specs['dnd2024:creatures/hydra']['fields']['splat_traits']['heads']['parameters']
        self.assertEqual(hydra['reaction_capacity_recomputed_at'],'owner_turn_start')
        self.assertEqual(hydra['healing_per_regrowth_event'],20)
        bag=specs['dnd2024:items/bag_of_holding']['fields']['splat_item']
        self.assertEqual(bag['containment']['capacity']['volume']['value'],64)
        self.assertEqual(bag['containment']['carried_mass']['value'],5)

    def test_catalogue_keeps_all_named_sources_and_separate_gap_classes(self):
        report=json.loads((OUTPUT/'coverage.json').read_text())
        records=report['entries']
        self.assertEqual(len({r['key'] for r in records}),len(records))
        self.assertEqual(len({r['id'] for r in records}),len(records))
        represented={r['source'] for r in records}
        for category in ['creature','spell','magic_item','glossary']:
            for path in (SOURCE/category).glob('*.md'):
                meta,_=load_note(path)
                self.assertIn(meta['splat_key'],represented)
        counts=collections.Counter(r['category'] for r in records)
        for category in ['class','subclass','table']:self.assertEqual(counts[category],12)
        for r in records:
            self.assertEqual(r['status']=='partial',bool(r['conversion_gaps']))
        self.assertEqual(report['counts']['executable_full_definitions'],0)

if __name__=='__main__':unittest.main()

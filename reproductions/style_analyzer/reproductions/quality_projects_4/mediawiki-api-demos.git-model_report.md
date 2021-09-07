# Model report for file:///tmp/top-repos-quality-repos-2vjtf8h8/mediawiki-api-demos.git HEAD d69d9d0ddd6f14a41a0657e68dc2f3cca274893c

### Dump

```json
{'created_at': '2021-09-02 16:06:30',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '21.8 kB',
 'tags': [],
 'uuid': '03d65615-715b-44e9-a7c8-4c8cbe945444',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2vjtf8h8/mediawiki-api-demos.git d69d9d0ddd6f14a41a0657e68dc2f3cca274893c

# javascript
34 rules, avg.len. 10.2
## train
PPCR: 0.893409
### report
macro
{'f1-score': 0.704759564337701,
 'precision': 0.7160822838901345,
 'recall': 0.6950275310486966,
 'support': 24047}
micro
{'f1-score': 0.9556701459641536,
 'precision': 0.9556701459641536,
 'recall': 0.9556701459641536,
 'support': 24047}
weighted
{'f1-score': 0.954462296648515,
 'precision': 0.9538618117074653,
 'recall': 0.9556701459641536,
 'support': 24047}
### report_full
macro
{'f1-score': 0.6268229629495096,
 'precision': 0.7160822838901345,
 'recall': 0.5674995508327934,
 'support': 26916}
micro
{'f1-score': 0.9018699841061162,
 'precision': 0.9556701459641536,
 'recall': 0.8538044285926586,
 'support': 26916}
weighted
{'f1-score': 0.891609130443517,
 'precision': 0.9398467520644572,
 'recall': 0.8538044285926586,
 'support': 26916}
## test
PPCR: 0.896860
### report
macro
{'f1-score': 0.7056874946834512,
 'precision': 0.7157304949874628,
 'recall': 0.6986576205086243,
 'support': 5626}
micro
{'f1-score': 0.9619623178101671,
 'precision': 0.9619623178101671,
 'recall': 0.9619623178101671,
 'support': 5626}
weighted
{'f1-score': 0.9613171714031102,
 'precision': 0.9613957445646762,
 'recall': 0.9619623178101671,
 'support': 5626}
### report_full
macro
{'f1-score': 0.6330542556488807,
 'precision': 0.7157304949874628,
 'recall': 0.5796786898209355,
 'support': 6273}
micro
{'f1-score': 0.9096562736364401,
 'precision': 0.9619623178101671,
 'recall': 0.8627450980392157,
 'support': 6273}
weighted
{'f1-score': 0.9013015529989512,
 'precision': 0.9512527767137028,
 'recall': 0.8627450980392157,
 'support': 6273}
```

## javascript
### Summary
26 rules, avg.len. 9.2

| | |
|-|-|
|Min support|91|
|Max support|4392|
|Min confidence|0.9224299192428589|
|Max confidence|0.9993917346000671|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -2.roles in {IDENTIFIER}<br>	∧ +1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1212.` |
| 2 | `  -2.roles not in {IDENTIFIER}<br>	∧ -3.roles in {VALUE}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 658.` |
| 3 | `  -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {VALUE}<br>	∧ -4.diff_offset ≥ 13<br>	∧ +1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.922. Support: 535.` |
| 4 | `  -1.roles in {STRING}<br>	∧ -5.length ≥ 5<br>	∧ +1.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.978. Support: 572.` |
| 5 | `  -1.roles in {STRING}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ -5.length ≤ 4<br>	∧ +1.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.980. Support: 228.` |
| 6 | `  -1.roles in {STRING}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ -5.length ≤ 4<br>	∧ +1.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.951. Support: 379.` |
| 7 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1117.` |
| 8 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<space>}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.932. Support: 169.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 701.` |
| 10 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved = (<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 188.` |
| 11 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FOR, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 4392.` |
| 12 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 102.` |
| 13 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved not in {}}<br>	∧ -5.diff_col ≤ 48<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 2014.` |
| 14 | `  •••start_line ≥ 37<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.980. Support: 226.` |
| 15 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 822.` |
| 16 | `  -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 395.` |
| 17 | `  -1.reserved not in {var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.977. Support: 109.` |
| 18 | `  -1.reserved not in {var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 105.` |
| 19 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.internal_type = Identifier<br>	∧ -4.label not in {<space>}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 115.` |
| 20 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +5.internal_type = Identifier<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 93.` |
| 21 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +5.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 162.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = var<br>	∧ +1.roles not in {MAP}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 164.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {;, var, }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label in {<newline>} and not in {<space>}<br>	∧ -5.reserved not in {}}<br>	∧ +1.reserved not in {;, var, }}<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 136.` |
| 25 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label in {<newline>} and not in {<space>}<br>	∧ -5.reserved not in {}}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {;, var, }}<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 26 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, var}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {)}<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_col ≤ 18<br>	∧ -5.label not in {<newline>, <space>}<br>	∧ -5.reserved not in {}}<br>	∧ -5.roles not in {FUNCTION, LITERAL}<br>	∧ +1.reserved not in {;, var, }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1734.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.153846153846153, "max_conf": 0.9993917346000671, "max_support": 4392, "min_conf": 0.9224299192428589, "min_support": 91, "num_rules": 26}}
```
</details>

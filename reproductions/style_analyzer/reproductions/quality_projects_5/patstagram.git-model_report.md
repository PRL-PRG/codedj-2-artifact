# Model report for file:///tmp/top-repos-quality-repos-_eymforx/patstagram.git HEAD d20b076a10b70613fc513008351572da4a1b43aa

### Dump

```json
{'created_at': '2021-09-02 11:31:34',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.1 kB',
 'tags': [],
 'uuid': 'ec6d5f53-7fb1-49e7-be81-54a0eb0d7eb3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_eymforx/patstagram.git d20b076a10b70613fc513008351572da4a1b43aa

# javascript
44 rules, avg.len. 7.2
## train
PPCR: 0.628594
### report
macro
{'f1-score': 0.2651374542352818,
 'precision': 0.2593764777372617,
 'recall': 0.27157845196912456,
 'support': 6494}
micro
{'f1-score': 0.881429011395134,
 'precision': 0.881429011395134,
 'recall': 0.881429011395134,
 'support': 6494}
weighted
{'f1-score': 0.8615732616428559,
 'precision': 0.8438394088771695,
 'recall': 0.881429011395134,
 'support': 6494}
### report_full
macro
{'f1-score': 0.20620074067477642,
 'precision': 0.2593764777372617,
 'recall': 0.17926210888412816,
 'support': 10331}
micro
{'f1-score': 0.6804160475482912,
 'precision': 0.881429011395134,
 'recall': 0.5540605943277515,
 'support': 10331}
weighted
{'f1-score': 0.6191577043962763,
 'precision': 0.7184061482268371,
 'recall': 0.5540605943277515,
 'support': 10331}
## test
PPCR: 0.629696
### report
macro
{'f1-score': 0.17454703500566415,
 'precision': 0.16760186875150418,
 'recall': 0.182214223337663,
 'support': 2095}
micro
{'f1-score': 0.8463007159904534,
 'precision': 0.8463007159904534,
 'recall': 0.8463007159904534,
 'support': 2095}
weighted
{'f1-score': 0.8161686122704945,
 'precision': 0.7885234628715175,
 'recall': 0.8463007159904534,
 'support': 2095}
### report_full
macro
{'f1-score': 0.1512033262063294,
 'precision': 0.16760186875150418,
 'recall': 0.1377679235470757,
 'support': 3327}
micro
{'f1-score': 0.6540022132054593,
 'precision': 0.8463007159904534,
 'recall': 0.5329125338142471,
 'support': 3327}
weighted
{'f1-score': 0.5872459916522925,
 'precision': 0.654116191054068,
 'recall': 0.5329125338142471,
 'support': 3327}
```

## javascript
### Summary
29 rules, avg.len. 6.7

| | |
|-|-|
|Min support|158|
|Max support|812|
|Min confidence|0.932624101638794|
|Max confidence|0.9980695247650146|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 91,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 179.` |
| 2 | `  -1.reserved not in {:, ;}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles not in {MAP, PATHNAME}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.967. Support: 686.` |
| 3 | `  -1.reserved not in {:, ;}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION} and not in {MAP, PATHNAME}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 186.` |
| 4 | `  •••start_line ≥ 8<br>	∧ -1.diff_col ≤ 36<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {ARGUMENT, EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 5 | `  •••start_line ≥ 8<br>	∧ -1.diff_col ≤ 36<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.roles not in {ARGUMENT, EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.955. Support: 411.` |
| 6 | `  +1.reserved = .<br>⇒ y = ∅<br>Confidence: 0.972. Support: 371.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.roles in {KEY}<br>	∧ -4.diff_offset ≤ 94<br>	∧ +1.reserved not in {.}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 266.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_offset ≤ 94<br>	∧ +1.reserved not in {., {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.internal_type not in {Program, VariableDeclarator}<br>	∧ ^1.roles not in {IMPORT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 259.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_offset ≤ 94<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 24<br>	∧ ^1.internal_type not in {Program, VariableDeclarator}<br>	∧ ^1.roles not in {IMPORT}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 158.` |
| 10 | `  -1.reserved = :<br>	∧ ^1.roles not in {IMPORT}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 306.` |
| 11 | `  ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 812.` |
| 12 | `  -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 330.` |
| 13 | `  -1.diff_offset ≤ 33<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles in {KEY}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 310.` |
| 14 | `  -1.diff_offset ≤ 33<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 273.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {COMMENT}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 251.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {COMMENT}<br>	∧ ^1.roles in {IDENTIFIER} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 503.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {COMMENT}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 545.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 262.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.diff_offset ≥ 5<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CALL, DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 423.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≤ 1<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BINARY, CALL, DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 161.` |
| 21 | `  -1.roles in {COMMENT}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 212.` |
| 22 | `  -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL, PATHNAME}<br>	∧ +2.reserved not in {=}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 224.` |
| 23 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.roles not in {IMPORT}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 747.` |
| 24 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION, IMPORT}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 677.` |
| 25 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION, IMPORT}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, MODULE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 235.` |
| 26 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = .<br>	∧ +1.roles not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 381.` |
| 27 | `  -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 377.` |
| 28 | `  -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.980. Support: 280.` |
| 29 | `  -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 507.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.724137931034483, "max_conf": 0.9980695247650146, "max_support": 812, "min_conf": 0.932624101638794, "min_support": 158, "num_rules": 29}}
```
</details>

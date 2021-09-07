# Model report for file:///tmp/top-repos-quality-repos-7s5cpqyw/prizey-web.git HEAD e8fa61071d69a295397656cd5e52645ba5782562

### Dump

```json
{'created_at': '2021-09-01 01:58:20',
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
 'uuid': 'f4d18cd8-a0cc-40f9-9b6d-0ddbce454b25',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7s5cpqyw/prizey-web.git e8fa61071d69a295397656cd5e52645ba5782562

# javascript
115 rules, avg.len. 9.0
## train
PPCR: 0.889931
### report
macro
{'f1-score': 0.822021842705086,
 'precision': 0.9175483478316719,
 'recall': 0.7622524054353755,
 'support': 25622}
micro
{'f1-score': 0.9096869877449067,
 'precision': 0.9096869877449067,
 'recall': 0.9096869877449067,
 'support': 25622}
weighted
{'f1-score': 0.9058553932829186,
 'precision': 0.9091030499273736,
 'recall': 0.9096869877449067,
 'support': 25622}
### report_full
macro
{'f1-score': 0.7142558178510532,
 'precision': 0.9175483478316719,
 'recall': 0.6351997209777707,
 'support': 28791}
micro
{'f1-score': 0.856707036921324,
 'precision': 0.9096869877449067,
 'recall': 0.8095585426001181,
 'support': 28791}
weighted
{'f1-score': 0.8366851219093282,
 'precision': 0.9058099914144331,
 'recall': 0.8095585426001181,
 'support': 28791}
## test
PPCR: 0.886429
### report
macro
{'f1-score': 0.825879389510775,
 'precision': 0.9128244009204807,
 'recall': 0.7675409050062184,
 'support': 6486}
micro
{'f1-score': 0.9108849830403947,
 'precision': 0.9108849830403947,
 'recall': 0.9108849830403947,
 'support': 6486}
weighted
{'f1-score': 0.9079844098816282,
 'precision': 0.9101518755463185,
 'recall': 0.9108849830403947,
 'support': 6486}
### report_full
macro
{'f1-score': 0.71052193674665,
 'precision': 0.9128244009204807,
 'recall': 0.6290651239617469,
 'support': 7317}
micro
{'f1-score': 0.8560457871477215,
 'precision': 0.9108849830403947,
 'recall': 0.8074347410140769,
 'support': 7317}
weighted
{'f1-score': 0.8377529591292097,
 'precision': 0.9077315571304904,
 'recall': 0.8074347410140769,
 'support': 7317}
```

## javascript
### Summary
73 rules, avg.len. 9.2

| | |
|-|-|
|Min support|137|
|Max support|10028|
|Min confidence|0.9217926263809204|
|Max confidence|0.9992614388465881|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.997. Support: 198.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.997. Support: 913.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 677.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.998. Support: 220.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1016.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 215.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 216.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 276.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 306.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.932. Support: 169.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = <<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/, const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 393.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, >}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/, const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 9345.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 202.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 559.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.998. Support: 204.` |
| 18 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.995. Support: 938.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 209.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 476.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.941. Support: 160.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 8604.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 137.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.998. Support: 203.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 985.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ +1.roles in {INCOMPLETE} and not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ +3.reserved = ><br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 290.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 162.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.internal_type = JSXIdentifier<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, MODULE, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 8686.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 499.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 339.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ +3.reserved = ><br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 274.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.961. Support: 191.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.internal_type = JSXIdentifier<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 198.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 9441.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 234.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.956. Support: 169.` |
| 41 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 221.` |
| 42 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.996. Support: 952.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved = =<br>	∧ +3.reserved = ><br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 288.` |
| 44 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.963. Support: 150.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = <<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/, const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 10028.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 197.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {/, const}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 569.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 248.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.961. Support: 190.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.internal_type = JSXIdentifier<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 8581.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.998. Support: 220.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.reserved not in {>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≥ 5<br>	∧ +1.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 195.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<space>}<br>	∧ -4.diff_col ≥ 5<br>	∧ +1.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 165.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≥ 5<br>	∧ +1.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 992.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 527.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {LITERAL}<br>	∧ +5.internal_type = Identifier<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 150.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.929. Support: 162.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 9600.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.998. Support: 230.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {CALLEE, LITERAL, MAP}<br>	∧ +2.internal_type = JSXIdentifier<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {CALLEE, LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 9876.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 233.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE} and not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 521.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {LITERAL}<br>	∧ +3.reserved = {<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION, FILE}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 151.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = <<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.reserved not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -5.label in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = File<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 565.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -5.label in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +5.internal_type = Identifier<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 185.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ -5.label not in {'}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {..., =}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 9373.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.label not in {'}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 227.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.150684931506849, "max_conf": 0.9992614388465881, "max_support": 10028, "min_conf": 0.9217926263809204, "min_support": 137, "num_rules": 73}}
```
</details>

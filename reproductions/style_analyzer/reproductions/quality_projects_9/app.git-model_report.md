# Model report for file:///tmp/top-repos-quality-repos-2hhfqoh6/app.git HEAD 0954093a6fa73a2f79926a7e5e1a3857fe09e3ea

### Dump

```json
{'created_at': '2021-08-31 20:19:08',
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
 'size': '23.6 kB',
 'tags': [],
 'uuid': '94beaf4e-9367-4cfa-a2f9-83ada5bc8aae',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2hhfqoh6/app.git 0954093a6fa73a2f79926a7e5e1a3857fe09e3ea

# javascript
217 rules, avg.len. 11.0
## train
PPCR: 0.968494
### report
macro
{'f1-score': 0.6721021345574593,
 'precision': 0.7114874279790985,
 'recall': 0.6445960522722025,
 'support': 58499}
micro
{'f1-score': 0.9230756081300535,
 'precision': 0.9230756081300535,
 'recall': 0.9230756081300535,
 'support': 58499}
weighted
{'f1-score': 0.9187884873547201,
 'precision': 0.9184393731656617,
 'recall': 0.9230756081300535,
 'support': 58499}
### report_full
macro
{'f1-score': 0.6547979997685692,
 'precision': 0.7114874279790985,
 'recall': 0.6167671968592745,
 'support': 60402}
micro
{'f1-score': 0.9083018645764124,
 'precision': 0.9230756081300535,
 'recall': 0.8939935763716433,
 'support': 60402}
weighted
{'f1-score': 0.901686724770898,
 'precision': 0.9155208614543824,
 'recall': 0.8939935763716433,
 'support': 60402}
## test
PPCR: 0.971512
### report
macro
{'f1-score': 0.6531690764729257,
 'precision': 0.7013072248341108,
 'recall': 0.6234244345492146,
 'support': 14630}
micro
{'f1-score': 0.9097060833902939,
 'precision': 0.9097060833902939,
 'recall': 0.9097060833902939,
 'support': 14630}
weighted
{'f1-score': 0.9039925777954635,
 'precision': 0.9045146512715841,
 'recall': 0.9097060833902939,
 'support': 14630}
### report_full
macro
{'f1-score': 0.6390343361256097,
 'precision': 0.7013072248341108,
 'recall': 0.6017461703020317,
 'support': 15059}
micro
{'f1-score': 0.8965610158644617,
 'precision': 0.9097060833902939,
 'recall': 0.8837904243309649,
 'support': 15059}
weighted
{'f1-score': 0.8888215458052097,
 'precision': 0.9021260391141372,
 'recall': 0.8837904243309649,
 'support': 15059}
```

## javascript
### Summary
124 rules, avg.len. 10.0

| | |
|-|-|
|Min support|145|
|Max support|11252|
|Min confidence|0.9200328588485718|
|Max confidence|0.9996688961982727|

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
| 1 | `  -1.reserved = }<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 523.` |
| 2 | `  -1.reserved not in {}}<br>	∧ -1.roles in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.992. Support: 192.` |
| 3 | `  -3.label in {<newline>}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 269.` |
| 4 | `  -3.label not in {<newline>}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 298.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 11252.` |
| 6 | `  -1.roles not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 3614.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 1371.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 895.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.952. Support: 1664.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 2937.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 1516.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 806.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 714.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 753.` |
| 15 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 505.` |
| 16 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 315.` |
| 17 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 368.` |
| 18 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 19 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 633.` |
| 20 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ -3.diff_offset ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {BLOCK, FILE, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 181.` |
| 21 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, FunctionDeclaration, ObjectExpression}<br>	∧ ^1.roles not in {BLOCK, FILE, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 8723.` |
| 22 | `  •••start_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 385.` |
| 23 | `  -4.diff_line ≥ 1<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 270.` |
| 24 | `  -4.diff_line = 0<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 354.` |
| 25 | `  -1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3798.` |
| 26 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.995. Support: 1376.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 854.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.942. Support: 1703.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2792.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1539.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 788.` |
| 32 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.roles in {IMPORT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 291.` |
| 33 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 282.` |
| 34 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 35 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 639.` |
| 36 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, ObjectExpression}<br>	∧ ^1.roles not in {BLOCK, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 300.` |
| 37 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, JSXFragment, MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {BLOCK, FILE, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 532.` |
| 38 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {BLOCK, FILE, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 163.` |
| 39 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, FunctionDeclaration, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {BLOCK, FILE, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 8535.` |
| 40 | `  -1.roles in {LITERAL}<br>	∧ -1.length ≥ 2<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.992. Support: 190.` |
| 41 | `  -1.reserved = }<br>	∧ -1.length ≤ 1<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 490.` |
| 42 | `  -1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 3722.` |
| 43 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 1373.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 885.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.956. Support: 1681.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2896.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1540.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 781.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {CallExpression, FunctionDeclaration}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 4214.` |
| 50 | `  -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 626.` |
| 51 | `  -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 2816.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {KEY}<br>	∧ -1.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 899.` |
| 53 | `  -1.reserved not in {:, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 145.` |
| 54 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {KEY}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 244.` |
| 55 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.992. Support: 180.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 237.` |
| 57 | `  -1.diff_col ≤ 19<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 3045.` |
| 58 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 230.` |
| 59 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1134.` |
| 60 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 465.` |
| 61 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 344.` |
| 62 | `  •••start_col ≤ 5<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ -3.diff_col = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 150.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1510.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 4485.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 987.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 719.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, >, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 500.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 69 | `  -1.roles in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.993. Support: 210.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 852.` |
| 71 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 2828.` |
| 72 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 246.` |
| 73 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 1164.` |
| 74 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -2.label in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 75 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 522.` |
| 76 | `  •••start_col ≤ 17<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.964. Support: 458.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1496.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 216.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 1316.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 480.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 4548.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 843.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 3150.` |
| 84 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 307.` |
| 85 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {CallExpression, FunctionDeclaration}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 4337.` |
| 86 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 335.` |
| 87 | `  -1.reserved not in {}}<br>	∧ -1.roles in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.980. Support: 176.` |
| 88 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 393.` |
| 89 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 305.` |
| 90 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 91 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 606.` |
| 92 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -1.length ≤ 1<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 226.` |
| 93 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {FILE, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 147.` |
| 94 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, FunctionDeclaration, ObjectExpression}<br>	∧ ^1.roles not in {FILE, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 8515.` |
| 95 | `  •••start_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 461.` |
| 96 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 150.` |
| 97 | `  -1.length ≥ 2<br>	∧ -2.reserved = =<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.991. Support: 160.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 2981.` |
| 99 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {STRING} and not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 240.` |
| 100 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type = JSXFragment<br>	∧ ^1.roles not in {FILE, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 166.` |
| 101 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {CALL} and not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {FILE, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 102 | `  •••start_col ≤ 18<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL, KEY, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {CallExpression, JSXFragment, MemberExpression}<br>	∧ ^1.roles not in {FILE, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 3659.` |
| 103 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 326.` |
| 104 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 461.` |
| 105 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 277.` |
| 106 | `  •••start_col ≤ 1<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 107 | `  -1.roles in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.989. Support: 233.` |
| 108 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 300.` |
| 109 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {CallExpression, FunctionDeclaration, MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 4249.` |
| 110 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 321.` |
| 111 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 264.` |
| 112 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles in {EXPRESSION} and not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, JSXFragment, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION, FILE, INCOMPLETE, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 837.` |
| 113 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, JSXFragment, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION, FILE, INCOMPLETE, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 2277.` |
| 114 | `  -1.diff_col ≥ 2<br>	∧ -1.roles in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.981. Support: 187.` |
| 115 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 436.` |
| 116 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.roles in {IMPORT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 286.` |
| 117 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 288.` |
| 118 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |
| 119 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 606.` |
| 120 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -1.length ≤ 1<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.961. Support: 220.` |
| 121 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 327.` |
| 122 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, JSXFragment, ObjectExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 553.` |
| 123 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 167.` |
| 124 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, FunctionDeclaration, ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 8759.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.991935483870968, "max_conf": 0.9996688961982727, "max_support": 11252, "min_conf": 0.9200328588485718, "min_support": 145, "num_rules": 124}}
```
</details>

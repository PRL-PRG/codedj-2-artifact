# Model report for file:///tmp/top-repos-quality-repos-idkzfw1t/guardioes-web.git HEAD 468d4f518ee2788021a0ef3d6af845b4b4eeb665

### Dump

```json
{'created_at': '2021-08-31 23:46:10',
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
 'size': '21.1 kB',
 'tags': [],
 'uuid': '8d95a5e1-1a7b-4b61-8666-b5854a68de1b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-idkzfw1t/guardioes-web.git 468d4f518ee2788021a0ef3d6af845b4b4eeb665

# javascript
44 rules, avg.len. 9.1
## train
PPCR: 0.898367
### report
macro
{'f1-score': 0.4513087257975861,
 'precision': 0.4605517852445442,
 'recall': 0.44562261955000837,
 'support': 54450}
micro
{'f1-score': 0.949880624426079,
 'precision': 0.949880624426079,
 'recall': 0.949880624426079,
 'support': 54450}
weighted
{'f1-score': 0.9423897380827186,
 'precision': 0.935759122257042,
 'recall': 0.949880624426079,
 'support': 54450}
### report_full
macro
{'f1-score': 0.41324791317310156,
 'precision': 0.4605517852445442,
 'recall': 0.38155343626318466,
 'support': 60610}
micro
{'f1-score': 0.8990265948200941,
 'precision': 0.949880624426079,
 'recall': 0.8533410328328659,
 'support': 60610}
weighted
{'f1-score': 0.873285360196575,
 'precision': 0.8975650805479872,
 'recall': 0.8533410328328659,
 'support': 60610}
## test
PPCR: 0.900010
### report
macro
{'f1-score': 0.4330057911083405,
 'precision': 0.44399620542029633,
 'recall': 0.4300903011507177,
 'support': 9406}
micro
{'f1-score': 0.9374867106102488,
 'precision': 0.9374867106102488,
 'recall': 0.9374867106102488,
 'support': 9406}
weighted
{'f1-score': 0.9293235331106215,
 'precision': 0.9240067283433849,
 'recall': 0.9374867106102488,
 'support': 9406}
### report_full
macro
{'f1-score': 0.3835082300549139,
 'precision': 0.44399620542029633,
 'recall': 0.35209856744853374,
 'support': 10451}
micro
{'f1-score': 0.8881502744624061,
 'precision': 0.9374867106102488,
 'recall': 0.8437470098555162,
 'support': 10451}
weighted
{'f1-score': 0.8603412529493641,
 'precision': 0.8846420192904833,
 'recall': 0.8437470098555162,
 'support': 10451}
```

## javascript
### Summary
29 rules, avg.len. 8.8

| | |
|-|-|
|Min support|99|
|Max support|8544|
|Min confidence|0.9251839518547058|
|Max confidence|0.9995421171188354|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.964. Support: 7477.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved = const<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 562.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles in {PATHNAME}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 4 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {PATHNAME}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 237.` |
| 5 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {PATHNAME}<br>	∧ +3.roles in {CALL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 134.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {CALL}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.986. Support: 104.` |
| 7 | `  -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {PATHNAME}<br>	∧ ^1.internal_type not in {JSXElement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 8544.` |
| 8 | `  •••start_line ≥ 88<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement, Program}<br>⇒ y = "<br>Confidence: 0.929. Support: 973.` |
| 9 | `  •••start_line ≤ 87<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement, Program}<br>	∧ ^2.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.997. Support: 179.` |
| 10 | `  •••start_line ≤ 87<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.label in {<space>}<br>	∧ ^1.internal_type not in {JSXElement, Program}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.973. Support: 131.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 628.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 447.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 9<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>⇒ y = "<br>Confidence: 0.925. Support: 1223.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2457.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2006.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1092.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1122.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 928.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = export<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎⏎<br>Confidence: 0.982. Support: 251.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, ;}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXElement, JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 485.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {}}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 99.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {}}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved = "<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>	∧ ^2.roles in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 116.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 485.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 499.` |
| 25 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 226.` |
| 26 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {BODY} and not in {IDENTIFIER, INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 132.` |
| 27 | `  •••start_col ≥ 31<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 254.` |
| 28 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, [, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BODY, FILE, IDENTIFIER, INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 6141.` |
| 29 | `  •••start_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎⏎<br>Confidence: 0.936. Support: 118.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.827586206896552, "max_conf": 0.9995421171188354, "max_support": 8544, "min_conf": 0.9251839518547058, "min_support": 99, "num_rules": 29}}
```
</details>

# Model report for file:///tmp/top-repos-quality-repos-t6zjqnfc/redux HEAD 902484ed735d38aec06683c847810a7218d8dba2

### Dump

```json
{'created_at': '2021-08-13 16:10:36',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.6 kB',
 'tags': [],
 'uuid': '821a54d9-d27d-4cb1-bb9c-4f3662a162e7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t6zjqnfc/redux 902484ed735d38aec06683c847810a7218d8dba2

# javascript
33 rules, avg.len. 7.7
## train
PPCR: 0.917716
### report
macro
{'f1-score': 0.6819400068687477,
 'precision': 0.7042645707535596,
 'recall': 0.6636866934977785,
 'support': 32946}
micro
{'f1-score': 0.9387786074181995,
 'precision': 0.9387786074181995,
 'recall': 0.9387786074181995,
 'support': 32946}
weighted
{'f1-score': 0.9354232842352583,
 'precision': 0.9337573134593934,
 'recall': 0.9387786074181995,
 'support': 32946}
### report_full
macro
{'f1-score': 0.6169089577169681,
 'precision': 0.7042645707535596,
 'recall': 0.5596220434460656,
 'support': 35900}
micro
{'f1-score': 0.8984980972024519,
 'precision': 0.9387786074181995,
 'recall': 0.8615320334261839,
 'support': 35900}
weighted
{'f1-score': 0.8898809975357319,
 'precision': 0.9304800503461712,
 'recall': 0.8615320334261839,
 'support': 35900}
## test
PPCR: 0.907526
### report
macro
{'f1-score': 0.6833575216322204,
 'precision': 0.7123341706704616,
 'recall': 0.6591114894416368,
 'support': 4956}
micro
{'f1-score': 0.9253430185633575,
 'precision': 0.9253430185633575,
 'recall': 0.9253430185633575,
 'support': 4956}
weighted
{'f1-score': 0.9215855537415905,
 'precision': 0.9200728226738047,
 'recall': 0.9253430185633575,
 'support': 4956}
### report_full
macro
{'f1-score': 0.6240759193440711,
 'precision': 0.7123341706704616,
 'recall': 0.5652973651298755,
 'support': 5461}
micro
{'f1-score': 0.8804838245176155,
 'precision': 0.9253430185633575,
 'recall': 0.8397729353598242,
 'support': 5461}
weighted
{'f1-score': 0.8731683753648134,
 'precision': 0.9190103507592141,
 'recall': 0.8397729353598242,
 'support': 5461}
```

## javascript
### Summary
33 rules, avg.len. 7.7

| | |
|-|-|
|Min support|101|
|Max support|4159|
|Min confidence|0.8045454621315002|
|Max confidence|0.9995958209037781|

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
               'min_samples_leaf_max': 130,
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 4159.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.976. Support: 358.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 996.` |
| 4 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 678.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.827. Support: 176.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 122.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.827. Support: 101.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.881. Support: 785.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.948. Support: 357.` |
| 10 | `  •••start_col ≤ 34<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.911. Support: 2081.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 336.` |
| 12 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.862. Support: 481.` |
| 13 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement, ObjectExpression}<br>	∧ ^1.roles in {MODULE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 241.` |
| 14 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, MODULE, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.887. Support: 1994.` |
| 15 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_col ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.822. Support: 121.` |
| 16 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≥ 1<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.805. Support: 110.` |
| 17 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.diff_col ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.856. Support: 344.` |
| 18 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_col = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 19 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.978. Support: 757.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 160.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXAttribute}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1237.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 258.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.851. Support: 185.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.876. Support: 817.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 174.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.849. Support: 222.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.913. Support: 1020.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.895. Support: 291.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 237.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 4112.` |
| 31 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.886. Support: 328.` |
| 32 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 2694.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.806. Support: 152.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.666666666666667, "max_conf": 0.9995958209037781, "max_support": 4159, "min_conf": 0.8045454621315002, "min_support": 101, "num_rules": 33}}
```
</details>

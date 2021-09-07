# Model report for file:///tmp/top-repos-quality-repos-piup7489/bewell-in-school.git HEAD b798cbaa2a23ba420554e1bed34f488df409b754

### Dump

```json
{'created_at': '2021-09-02 02:54:09',
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
 'size': '16.7 kB',
 'tags': [],
 'uuid': 'd76c0643-740d-405f-aa4e-c66982b89b96',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-piup7489/bewell-in-school.git b798cbaa2a23ba420554e1bed34f488df409b754

# javascript
14 rules, avg.len. 5.5
## train
PPCR: 0.838012
### report
macro
{'f1-score': 0.548405964670851,
 'precision': 0.5646911178197049,
 'recall': 0.5354122064429316,
 'support': 7522}
micro
{'f1-score': 0.9113267747939378,
 'precision': 0.9113267747939378,
 'recall': 0.9113267747939378,
 'support': 7522}
weighted
{'f1-score': 0.8956434904049633,
 'precision': 0.8820129927598407,
 'recall': 0.9113267747939378,
 'support': 7522}
### report_full
macro
{'f1-score': 0.5144890235762025,
 'precision': 0.5646911178197049,
 'recall': 0.4745027022284777,
 'support': 8976}
micro
{'f1-score': 0.8310098193720451,
 'precision': 0.9113267747939378,
 'recall': 0.7637032085561497,
 'support': 8976}
weighted
{'f1-score': 0.7966319590579688,
 'precision': 0.8354089408831773,
 'recall': 0.7637032085561497,
 'support': 8976}
## test
PPCR: 0.815075
### report
macro
{'f1-score': 0.5503307443508731,
 'precision': 0.5795488279414653,
 'recall': 0.5323578248235783,
 'support': 1741}
micro
{'f1-score': 0.9040781160252729,
 'precision': 0.9040781160252729,
 'recall': 0.9040781160252729,
 'support': 1741}
weighted
{'f1-score': 0.8845328658880748,
 'precision': 0.8697457539183359,
 'recall': 0.9040781160252729,
 'support': 1741}
### report_full
macro
{'f1-score': 0.5026232247430769,
 'precision': 0.5795488279414653,
 'recall': 0.4477137363193225,
 'support': 2136}
micro
{'f1-score': 0.811968016507609,
 'precision': 0.9040781160252729,
 'recall': 0.7368913857677902,
 'support': 2136}
weighted
{'f1-score': 0.7806458557812754,
 'precision': 0.8340833342611922,
 'recall': 0.7368913857677902,
 'support': 2136}
```

## javascript
### Summary
9 rules, avg.len. 4.7

| | |
|-|-|
|Min support|94|
|Max support|1753|
|Min confidence|0.9270696640014648|
|Max confidence|0.996268630027771|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -2.reserved = =<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.995. Support: 94.` |
| 2 | `  -2.reserved not in {=}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1753.` |
| 3 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 761.` |
| 4 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.955. Support: 146.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.946. Support: 139.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = {<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 98.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 122.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 96.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.996268630027771, "max_support": 1753, "min_conf": 0.9270696640014648, "min_support": 94, "num_rules": 9}}
```
</details>

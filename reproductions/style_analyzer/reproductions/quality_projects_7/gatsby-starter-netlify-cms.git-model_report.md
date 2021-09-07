# Model report for file:///tmp/top-repos-quality-repos-o7zgu7pr/gatsby-starter-netlify-cms.git HEAD 4cb6da362de937d9974803d8ff32cff4292ec3a3

### Dump

```json
{'created_at': '2021-09-01 02:24:14',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '9323a82c-a94e-4794-9f48-0b8125b1bf94',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o7zgu7pr/gatsby-starter-netlify-cms.git 4cb6da362de937d9974803d8ff32cff4292ec3a3

# javascript
39 rules, avg.len. 4.8
## train
PPCR: 0.767356
### report
macro
{'f1-score': 0.46274930747760623,
 'precision': 0.47314119576443253,
 'recall': 0.45556282920737434,
 'support': 5637}
micro
{'f1-score': 0.9228312932410857,
 'precision': 0.9228312932410857,
 'recall': 0.9228312932410857,
 'support': 5637}
weighted
{'f1-score': 0.9054464105644319,
 'precision': 0.8932012973113758,
 'recall': 0.9228312932410857,
 'support': 5637}
### report_full
macro
{'f1-score': 0.4175866758009217,
 'precision': 0.47314119576443253,
 'recall': 0.3900075276833444,
 'support': 7346}
micro
{'f1-score': 0.8013556188862359,
 'precision': 0.9228312932410857,
 'recall': 0.7081404846174789,
 'support': 7346}
weighted
{'f1-score': 0.7437583205182043,
 'precision': 0.8197981337701277,
 'recall': 0.7081404846174789,
 'support': 7346}
## test
PPCR: 0.769714
### report
macro
{'f1-score': 0.4545667175397073,
 'precision': 0.4651844070961718,
 'recall': 0.44673982828050496,
 'support': 1347}
micro
{'f1-score': 0.9161098737936154,
 'precision': 0.9161098737936154,
 'recall': 0.9161098737936154,
 'support': 1347}
weighted
{'f1-score': 0.8967850964395937,
 'precision': 0.8823092364884584,
 'recall': 0.9161098737936154,
 'support': 1347}
### report_full
macro
{'f1-score': 0.4163853800284754,
 'precision': 0.4651844070961718,
 'recall': 0.3924861368181581,
 'support': 1750}
micro
{'f1-score': 0.7969002260251857,
 'precision': 0.9161098737936154,
 'recall': 0.7051428571428572,
 'support': 1750}
weighted
{'f1-score': 0.7381395516238333,
 'precision': 0.8072866213151927,
 'recall': 0.7051428571428572,
 'support': 1750}
```

## javascript
### Summary
25 rules, avg.len. 3.8

| | |
|-|-|
|Min support|136|
|Max support|1599|
|Min confidence|0.9491978883743286|
|Max confidence|0.9980988502502441|

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
| 1 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1599.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 253.` |
| 3 | `  -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 4 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 907.` |
| 5 | `  -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 187.` |
| 6 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 7 | `  -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 192.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.997. Support: 176.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.998. Support: 202.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1583.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 152.` |
| 13 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 254.` |
| 14 | `  -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 167.` |
| 15 | `  -1.label in {<space>}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 159.` |
| 16 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 845.` |
| 17 | `  -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 156.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.996. Support: 136.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 883.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 190.` |
| 21 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.997. Support: 184.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 157.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1595.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 260.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.76, "max_conf": 0.9980988502502441, "max_support": 1599, "min_conf": 0.9491978883743286, "min_support": 136, "num_rules": 25}}
```
</details>

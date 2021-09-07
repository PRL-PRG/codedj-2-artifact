# Model report for file:///tmp/top-repos-quality-repos-ocw146us/gatsby-starter-netlify-cms1.git HEAD 7c1d09728f585d13ce558891541b3f8630b24999

### Dump

```json
{'created_at': '2021-09-02 11:43:17',
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
 'size': '13.4 kB',
 'tags': [],
 'uuid': '1177a25f-7558-43bc-ae9c-ae785ae72bd5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ocw146us/gatsby-starter-netlify-cms1.git 7c1d09728f585d13ce558891541b3f8630b24999

# javascript
5 rules, avg.len. 1.6
## train
PPCR: 0.449878
### report
macro
{'f1-score': 0.19649769585253457,
 'precision': 0.1931159420289855,
 'recall': 0.2,
 'support': 1104}
micro
{'f1-score': 0.9655797101449275,
 'precision': 0.9655797101449275,
 'recall': 0.9655797101449275,
 'support': 1104}
weighted
{'f1-score': 0.9486709410271823,
 'precision': 0.9323441766435623,
 'recall': 0.9655797101449275,
 'support': 1104}
### report_full
macro
{'f1-score': 0.15550692924872356,
 'precision': 0.1931159420289855,
 'recall': 0.13015873015873017,
 'support': 2454}
micro
{'f1-score': 0.599213041034289,
 'precision': 0.9655797101449275,
 'recall': 0.4343928280358598,
 'support': 2454}
weighted
{'f1-score': 0.5189901183973292,
 'precision': 0.6445067502923355,
 'recall': 0.4343928280358598,
 'support': 2454}
## test
PPCR: 0.560748
### report
macro
{'f1-score': 0.19130434782608696,
 'precision': 0.18333333333333332,
 'recall': 0.2,
 'support': 60}
micro
{'f1-score': 0.9166666666666666,
 'precision': 0.9166666666666666,
 'recall': 0.9166666666666666,
 'support': 60}
weighted
{'f1-score': 0.8768115942028986,
 'precision': 0.8402777777777778,
 'recall': 0.9166666666666666,
 'support': 60}
### report_full
macro
{'f1-score': 0.1746031746031746,
 'precision': 0.18333333333333332,
 'recall': 0.16666666666666669,
 'support': 107}
micro
{'f1-score': 0.6586826347305389,
 'precision': 0.9166666666666666,
 'recall': 0.514018691588785,
 'support': 107}
weighted
{'f1-score': 0.5384957721406319,
 'precision': 0.5654205607476636,
 'recall': 0.514018691588785,
 'support': 107}
```

## javascript
### Summary
5 rules, avg.len. 1.6

| | |
|-|-|
|Min support|222|
|Max support|667|
|Min confidence|0.9467766284942627|
|Max confidence|0.9979338645935059|

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
                     'max_depth': 10,
                     'min_samples_leaf': 109,
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
| 1 | `  ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.973. Support: 647.` |
| 2 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 231.` |
| 3 | `  ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 667.` |
| 4 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.984. Support: 222.` |
| 5 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 242.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.6, "max_conf": 0.9979338645935059, "max_support": 667, "min_conf": 0.9467766284942627, "min_support": 222, "num_rules": 5}}
```
</details>

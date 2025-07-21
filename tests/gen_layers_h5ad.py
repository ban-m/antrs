"""Script to generate a sample h5ad file with raw X and additional layers.

# Usage
# python gen_layers_h5ad.py <output_file.h5ad>
"""

import anndata as ad
import numpy as np
import sys

def main():
    import pandas as pd
    # Generate random data
    n_obs, n_vars = 100, 20
    X = np.random.rand(n_obs, n_vars)  # Raw dense data

    # Create gene ids and barcode ids
    gene_ids = [f"gene_{i:03d}" for i in range(n_vars)]
    barcode_ids = [f"barcode_{i:03d}" for i in range(n_obs)]

    # Create var DataFrame
    var = pd.DataFrame({
        "genes": gene_ids,
        "is_hvg": np.random.choice([True, False], size=n_vars)
    }, index=gene_ids)

    # Create obs DataFrame
    obs = pd.DataFrame({
        "library": np.random.choice(["libA", "libB"], size=n_obs),
        "batch": np.random.choice(["batch1", "batch2"], size=n_obs)
    }, index=barcode_ids)

    adata = ad.AnnData(X, var=var, obs=obs)

    # Add layers with modified results
    adata.layers["log1p"] = np.log1p(X)
    adata.layers["sqrt"] = np.sqrt(X)

    # Save to file
    if len(sys.argv) < 2:
        print("Usage: python gen_layers_h5ad.py <output_file.h5ad>")
        sys.exit(1)
    output_file = sys.argv[1]
    adata.write(output_file)
    print(f"Layers h5ad file written to {output_file}")

if __name__ == "__main__":
    main()

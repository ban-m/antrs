"""Script to generate a sample h5ad file with compressed X matrix in AnnData.

# Usage
# python gen_compressed_h5ad.py <output_file.h5ad>
"""

import anndata as ad
import numpy as np
import scipy.sparse as sp
import sys

def main():
    import pandas as pd
    # Generate random data
    n_obs, n_vars = 100, 20
    X = sp.csr_matrix(np.random.rand(n_obs, n_vars))  # Compressed sparse row

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

    # Save to file
    if len(sys.argv) < 2:
        print("Usage: python gen_compressed_h5ad.py <output_file.h5ad>")
        sys.exit(1)
    output_file = sys.argv[1]
    adata.write(output_file)
    print(f"Compressed h5ad file written to {output_file}")

if __name__ == "__main__":
    main()

interface LogoProps {
    width?: string;
    height?: string;
    fill?: string;
}

const Logo: React.FC<LogoProps> = ({ fill = "#FFF" }) => (
    <svg width="344" height="56" viewBox="0 0 344 56" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path
            d="M328.513 18.008H335.209C336.329 18.008 337.409 18.184 338.449 18.536C339.489 18.872 340.409 19.392 341.209 20.096C342.009 20.8 342.649 21.688 343.129 22.76C343.609 23.816 343.849 25.064 343.849 26.504C343.849 27.96 343.569 29.224 343.009 30.296C342.465 31.352 341.753 32.232 340.873 32.936C340.009 33.624 339.041 34.144 337.969 34.496C336.913 34.832 335.873 35 334.849 35H328.513V18.008ZM333.793 32.264C334.737 32.264 335.625 32.16 336.457 31.952C337.305 31.728 338.041 31.392 338.665 30.944C339.289 30.48 339.777 29.888 340.129 29.168C340.497 28.432 340.681 27.544 340.681 26.504C340.681 25.48 340.521 24.6 340.201 23.864C339.881 23.128 339.433 22.536 338.857 22.088C338.297 21.624 337.625 21.288 336.841 21.08C336.073 20.856 335.225 20.744 334.297 20.744H331.537V32.264H333.793Z"
            fill={fill}
        />
        <path
            d="M316.689 18.008H319.305L326.625 35H323.169L321.585 31.112H314.217L312.681 35H309.297L316.689 18.008ZM320.529 28.52L317.913 21.608L315.249 28.52H320.529Z"
            fill={fill}
        />
        <path
            d="M298.302 18.008H304.014C304.83 18.008 305.614 18.088 306.366 18.248C307.134 18.408 307.806 18.68 308.382 19.064C308.958 19.432 309.414 19.936 309.75 20.576C310.102 21.2 310.278 21.984 310.278 22.928C310.278 24 310.094 24.864 309.726 25.52C309.358 26.176 308.862 26.688 308.238 27.056C307.614 27.408 306.886 27.648 306.054 27.776C305.238 27.904 304.382 27.968 303.486 27.968H301.326V35H298.302V18.008ZM303.15 25.376C303.582 25.376 304.03 25.36 304.494 25.328C304.958 25.296 305.382 25.2 305.766 25.04C306.166 24.88 306.486 24.64 306.726 24.32C306.982 24 307.11 23.552 307.11 22.976C307.11 22.448 306.998 22.032 306.774 21.728C306.55 21.408 306.262 21.168 305.91 21.008C305.558 20.832 305.166 20.72 304.734 20.672C304.302 20.624 303.886 20.6 303.486 20.6H301.326V25.376H303.15Z"
            fill={fill}
        />
        <path
            d="M280.513 18.008H283.537V24.776H291.481V18.008H294.505V35H291.481V27.512H283.537V35H280.513V18.008Z"
            fill={fill}
        />
        <path
            d="M275.748 21.992C275.124 21.32 274.516 20.872 273.924 20.648C273.348 20.424 272.764 20.312 272.172 20.312C271.292 20.312 270.492 20.472 269.772 20.792C269.068 21.096 268.46 21.528 267.948 22.088C267.436 22.632 267.036 23.272 266.748 24.008C266.476 24.744 266.34 25.536 266.34 26.384C266.34 27.296 266.476 28.136 266.748 28.904C267.036 29.672 267.436 30.336 267.948 30.896C268.46 31.456 269.068 31.896 269.772 32.216C270.492 32.536 271.292 32.696 272.172 32.696C272.86 32.696 273.524 32.536 274.164 32.216C274.82 31.88 275.428 31.352 275.988 30.632L278.484 32.408C277.716 33.464 276.78 34.232 275.676 34.712C274.572 35.192 273.396 35.432 272.148 35.432C270.836 35.432 269.628 35.224 268.524 34.808C267.436 34.376 266.492 33.776 265.692 33.008C264.908 32.224 264.292 31.288 263.844 30.2C263.396 29.112 263.172 27.904 263.172 26.576C263.172 25.216 263.396 23.984 263.844 22.88C264.292 21.76 264.908 20.808 265.692 20.024C266.492 19.24 267.436 18.64 268.524 18.224C269.628 17.792 270.836 17.576 272.148 17.576C273.3 17.576 274.364 17.784 275.34 18.2C276.332 18.6 277.252 19.288 278.1 20.264L275.748 21.992Z"
            fill={fill}
        />
        <path
            d="M244.982 18.008H248.99L257.222 30.632H257.27V18.008H260.294V35H256.454L248.054 21.968H248.006V35H244.982V18.008Z"
            fill={fill}
        />
        <path
            d="M241.27 28.76C241.27 29.784 241.094 30.712 240.742 31.544C240.39 32.36 239.91 33.056 239.302 33.632C238.694 34.208 237.974 34.656 237.142 34.976C236.31 35.28 235.406 35.432 234.43 35.432C233.454 35.432 232.55 35.28 231.718 34.976C230.886 34.656 230.158 34.208 229.534 33.632C228.926 33.056 228.446 32.36 228.094 31.544C227.758 30.712 227.59 29.784 227.59 28.76V18.008H230.614V28.664C230.614 29.08 230.678 29.52 230.806 29.984C230.934 30.432 231.142 30.848 231.43 31.232C231.734 31.616 232.126 31.936 232.606 32.192C233.102 32.432 233.71 32.552 234.43 32.552C235.15 32.552 235.75 32.432 236.23 32.192C236.726 31.936 237.118 31.616 237.406 31.232C237.71 30.848 237.926 30.432 238.054 29.984C238.182 29.52 238.246 29.08 238.246 28.664V18.008H241.27V28.76Z"
            fill={fill}
        />
        <path
            d="M215.837 18.008H218.453L225.773 35H222.317L220.733 31.112H213.365L211.829 35H208.445L215.837 18.008ZM219.677 28.52L217.061 21.608L214.397 28.52H219.677Z"
            fill={fill}
        />
        <path d="M197.896 18.008H200.92V32.264H208.144V35H197.896V18.008Z" fill={fill} />
        <path
            d="M115.148 30.8698L118.671 19.301L122.194 30.8698H115.148ZM114.869 8.29834L101.728 47.1145H110.134L112.908 38.14H124.434L127.208 47.1145H135.614L122.472 8.29834H114.869Z"
            fill={fill}
        />
        <path d="M176.445 8H184.258V46.8162H176.445V8Z" fill={fill} />
        <path
            d="M147.87 24.6427V15.2703H154.515C157.546 15.2703 159.147 16.8907 159.147 19.9569C159.147 23.0225 157.546 24.6427 154.515 24.6427H147.87ZM155.777 31.8769C162.953 31.5006 167.069 27.156 167.069 19.9569C167.069 12.4699 162.477 8 154.787 8H140.057V46.8159H147.87V34.6857L158.571 46.8159H168.634L155.354 31.8991L155.777 31.8769Z"
            fill={fill}
        />
        <path d="M82.5313 47.1145H90.3446V15.5689H103.461V8.29834H69.4141V15.5689H82.5313V47.1145Z" fill={fill} />
        <path
            d="M49.489 18.4517L49.4826 25.1813L9.9661 15.0154L23.3114 6.31705L49.489 18.4517ZM25.5188 46.7641L25.5086 24.732L46.5851 30.1605L25.5188 46.7641ZM20.0033 44.6529L5.51421 28.4097L5.50562 19.5436L19.9803 23.3081L20.0033 44.6529ZM0 14.9387L0.00212257 30.5344L22.7523 56L54.9548 30.6016L55 14.9151L22.826 0L0 14.9387Z"
            fill={fill}
        />
    </svg>
);

export default Logo;